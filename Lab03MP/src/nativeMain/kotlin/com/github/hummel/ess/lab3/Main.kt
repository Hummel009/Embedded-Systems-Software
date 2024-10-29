package com.github.hummel.ess.lab3

import kotlinx.cinterop.*
import platform.windows.*
import kotlin.math.max
import kotlin.math.sin

const val rgbWhite: COLORREF = 0x00FFFFFFu
const val POINT_COUNT: Int = 1200
const val AMPLITUDE: Double = 9.0
const val FREQUENCY: Double = 1.0

lateinit var points: MutableList<Point>
var timeOffset: Double = 0.0
var isRunning = true

data class Point(val x: Int, val y: Int)

fun main() {
	memScoped {
		val className = "STM32 Connector"
		val windowTitle = "WinAPI"

		points = MutableList(POINT_COUNT) { Point(0, 0) }

		val windowClass = alloc<WNDCLASSW>()
		windowClass.style = 0u
		windowClass.lpfnWndProc = staticCFunction(::wndProc)
		windowClass.cbClsExtra = 0
		windowClass.cbWndExtra = 0
		windowClass.hInstance = null
		windowClass.hIcon = null
		windowClass.hCursor = null
		windowClass.hbrBackground = (COLOR_WINDOW + 1).toLong().toCPointer()
		windowClass.lpszMenuName = null
		windowClass.lpszClassName = className.wcstr.ptr

		RegisterClassW(windowClass.ptr)

		val screenWidth = GetSystemMetrics(SM_CXSCREEN)
		val screenHeight = GetSystemMetrics(SM_CYSCREEN)

		val windowWidth = 1200
		val windowHeight = 670

		val windowX = max(0, (screenWidth - windowWidth) / 2)
		val windowY = max(0, (screenHeight - windowHeight) / 2)

		CreateWindowExW(
			0u,
			className,
			windowTitle,
			(WS_VISIBLE or WS_CAPTION or WS_SYSMENU).toUInt(),
			windowX,
			windowY,
			windowWidth,
			windowHeight,
			null,
			null,
			null,
			null
		)

		CreateThread(null, 0u, staticCFunction(::threadOperate), null, 0u, null)

		val msg = alloc<MSG>()
		while (GetMessageW(msg.ptr, null, 0u, 0u) != 0) {
			TranslateMessage(msg.ptr)
			DispatchMessageW(msg.ptr)
		}
	}
}

private fun wndProc(window: HWND?, msg: UINT, wParam: WPARAM, lParam: LPARAM): LRESULT {
	memScoped {
		when (msg.toInt()) {
			WM_PAINT -> {
				val paintStructure = alloc<PAINTSTRUCT>()
				val deviceContext = BeginPaint(window, paintStructure.ptr)

				val brush = CreateSolidBrush(rgbWhite)
				val square = alloc<RECT>()
				GetClientRect(window, square.ptr)
				FillRect(deviceContext, square.ptr, brush)

				for (i in points.indices) {
					if (i < points.size - 1) {
						MoveToEx(deviceContext, points[i].x, points[i].y, null)
						LineTo(deviceContext, points[i + 1].x, points[i + 1].y)
					}
				}

				DeleteObject(brush)
				EndPaint(window, paintStructure.ptr)
			}

			WM_CLOSE -> {
				isRunning = false // Остановить генерацию при закрытии окна
				DestroyWindow(window)
			}

			WM_DESTROY -> PostQuitMessage(0)
		}
	}
	return DefWindowProcW(window, msg, wParam, lParam)
}

@Suppress("UNUSED_PARAMETER")
fun threadOperate(lpParameter: LPVOID?): DWORD {
	while (isRunning) {
		timeOffset += FREQUENCY
		points = MutableList(POINT_COUNT) { index ->
			Point(calculateX(index), calculateY(index))
		}

		InvalidateRect(GetForegroundWindow(), null, TRUE)

		Sleep(100u)
	}
	return 0u
}

private fun calculateX(index: Int): Int = index * (1200 / POINT_COUNT)

private fun calculateY(index: Int): Int =
	((20 * AMPLITUDE * sin((index * FREQUENCY) + timeOffset)).toInt() + (670 / 2)).coerceIn(0..670)