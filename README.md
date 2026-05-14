# Simulador de Planificación de Procesos: Algoritmo SRTF

Una aplicación interactiva basada en terminal (TUI) para simular y visualizar el algoritmo de planificación de procesos **SRTF (Shortest Remaining Time First)**, un algoritmo expulsivo utilizado en Sistemas Operativos.

Este proyecto permite a estudiantes y desarrolladores experimentar visualmente cómo el sistema operativo decide qué proceso toma el control de la CPU, cómo se calculan las penalizaciones por cambios de contexto y cómo se comportan los tiempos muertos de la CPU.

## 🚀 Características Principales

- **Simulación Interactiva SRTF**: Calcula la ejecución de procesos basándose en el tiempo restante más corto. Soporta expropiación (preemption).
- **Interfaz Gráfica en Terminal (TUI)**: Diseño moderno dividido por bloques funcionales para una experiencia de usuario amigable.
- **Tabla de Procesos Dinámica**: Permite agregar, editar (modificando IDs, tiempos de llegada y ráfagas) y eliminar procesos de manera interactiva directamente desde la interfaz.
- **Gráfica de Gantt Paso a Paso**: Visualiza de forma animada qué proceso está usando la CPU en cada instante, dibujando bloques proporcionales de ejecución e identificando tiempos muertos.
- **Cálculos Precisos en Tiempo Real**: 
  - Tiempos de Espera (TE) y de Finalización por proceso.
  - Tiempo de Espera Promedio (TEP).
  - Tiempo Total de Procesamiento (TTP) ajustado al reloj absoluto.
  - Penalizaciones estrictas por Cambios de Contexto.
- **Soporte de Ratón y Teclado**: Navega fácilmente por la tabla con flechas, confirma con Enter y avanza la simulación haciendo click en el botón "Next Step".

## 🛠️ Tecnologías y Librerías Utilizadas

El simulador está desarrollado enteramente en **Rust**, garantizando rendimiento, seguridad de memoria y concurrencia. Se apoyó en el siguiente stack de librerías:

* **[Ratatui](https://github.com/ratatui-org/ratatui)**: La librería principal para construir la interfaz de usuario en la terminal. Proporciona los widgets, layouts de cajas, estados interactivos y manejo de estilos (colores, bordes).
* **[Crossterm](https://github.com/crossterm-rs/crossterm)**: Utilizado como *backend* de Ratatui para la manipulación y renderizado en la terminal. Además, captura eficientemente los eventos de entrada del usuario (teclas y clicks del ratón).
* **[color-eyre](https://github.com/eyre-rs/color-eyre)**: Utilizado para el manejo de errores limpios y reportes de pánico (panics) visualmente amigables en la terminal, lo que facilita el desarrollo y el debugueo.

## 🧠 Estructura del Proyecto y Arquitectura

El proyecto sigue una arquitectura fuertemente orientada a componentes que interactúan a través del Patrón Elm (Modelo - Vista - Actualización):

* **`src/app.rs`**: El corazón de la aplicación. Maneja el estado global, el bucle principal (Game Loop) y enruta los eventos a los componentes correspondientes.
* **`src/components/`**: Los componentes visuales modulares:
  * `table.rs`: Mantiene el estado de edición e interactividad de la lista de procesos.
  * `grafica_gantt.rs`: Procesa el avance lógico del tiempo y dibuja el diagrama de Gantt basándose en los resultados matemáticos.
  * `buttons.rs`: Lógica para los elementos clickeables que controlan el flujo de la simulación.
* **`src/core/`**: El motor lógico matemático puro de la simulación.
  * `srtf_Algorithm.rs`: Implementa toda la lógica matemática, reglas de negocio de los cambios de contexto, tiempos de espera, turnaround time y selección de procesos. Separado intencionalmente de la UI para garantizar que sea 100% testeable mediante pruebas unitarias.
  * `process.rs`: Define las estructuras y abstracciones de un Proceso dentro de un S.O.

## ⌨️ Controles de la Aplicación

- **Tabla de Procesos**:
  - Usa las `Flechas (Arriba, Abajo, Izquierda, Derecha)` para navegar por las celdas.
  - Presiona `Enter` para entrar en Modo de Edición en una celda. Escribe el número y vuelve a presionar `Enter` para confirmar, o `Esc` para cancelar.
  - Usa los atajos (Añadir/Eliminar) si están disponibles en la tabla.
  - Guarda los cambios para reiniciar el simulador con tu nueva configuración.
- **Botón Paso**: Haz *click* con tu ratón en el botón inferior para avanzar un bloque de ejecución a la vez y ver crecer la gráfica de Gantt.
- **Teclas Globales**:
  - `q` o `Q`: Salir de la aplicación limpiamente.

## ⚙️ Cómo Ejecutar

Asegúrate de tener Rust instalado en tu sistema.
1. Clona el repositorio.
2. Navega a la raíz del proyecto.
3. Ejecuta la aplicación mediante Cargo:
```bash
cargo run
```
Para ejecutar la suite de pruebas matemáticas del algoritmo (que validan los tiempos exactos y los cambios de contexto):
```bash
cargo test
```
