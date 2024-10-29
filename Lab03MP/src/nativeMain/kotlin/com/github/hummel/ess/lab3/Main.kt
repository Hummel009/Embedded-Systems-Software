package com.github.hummel.ess.lab3

import kotlinx.cinterop.*
import platform.windows.*
import kotlin.math.max
import kotlin.random.Random

const val rgbWhite: COLORREF = 0x00FFFFFFu

lateinit var points: List<Point>

data class Point(val x: Int, val y: Int)

fun main() {
	memScoped {
		val className = "STM32 Connector"
		val windowTitle = "WinAPI"

		points = List(100) { index ->
			Point(index * (1200 / 100), Random.nextInt(0, 670))
		}

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
				clearAndUpdate(window)

				val paintStructure = alloc<PAINTSTRUCT>()
				val deviceContext = BeginPaint(window, paintStructure.ptr)

				for (i in points.indices) {
					if (i < points.size - 1) {
						MoveToEx(deviceContext, points[i].x, points[i].y, null)
						LineTo(deviceContext, points[i + 1].x, points[i + 1].y)
					}
				}

				EndPaint(window, paintStructure.ptr)
			}

			WM_CLOSE -> DestroyWindow(window)
			WM_DESTROY -> PostQuitMessage(0)
		}
	}
	return DefWindowProcW(window, msg, wParam, lParam)
}

private fun clearAndUpdate(window: HWND?) {
	memScoped {
		val deviceContext = GetDC(window)
		val square = alloc<RECT>()
		val brush = CreateSolidBrush(rgbWhite)

		GetClientRect(window, square.ptr)

		FillRect(deviceContext, square.ptr, brush)

		InvalidateRect(window, null, TRUE)

		ReleaseDC(window, deviceContext)
	}
}