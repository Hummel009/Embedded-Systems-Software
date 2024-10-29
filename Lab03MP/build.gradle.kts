import java.time.LocalDate
import java.time.format.DateTimeFormatter

plugins {
	id("org.jetbrains.kotlin.multiplatform") version "latest.release"
}

group = "com.github.hummel"
version = LocalDate.now().format(DateTimeFormatter.ofPattern("yy.MM.dd"))

kotlin {
	mingwX64 {
		binaries {
			executable {
				entryPoint("com.github.hummel.ess.lab3.main")
				linkerOpts("-lwinmm")
				baseName = "${project.name}-${project.version}"
			}
		}
	}
	sourceSets {
		configureEach {
			languageSettings {
				optIn("kotlinx.cinterop.ExperimentalForeignApi")
			}
		}
	}
}