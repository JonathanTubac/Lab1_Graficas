# Lab 1 — Gráficas por Computadora

Rasterizador básico escrito en Rust, sin usar las funciones de dibujo de ninguna
librería gráfica. Todo se dibuja pixel por pixel sobre un framebuffer propio y se
exporta a imagen.

![Resultado](out.png)

## Qué hace

Implementa desde cero la cadena completa de primitivas de dibujo, cada una
construida sobre la anterior:

| Primitiva | Implementación |
|---|---|
| **Punto** | Escribe un pixel directo en el framebuffer |
| **Línea** | Algoritmo de **Bresenham** (solo enteros, sin flotantes en el bucle) |
| **Polígono** | Contorno cerrado: una línea entre cada par de vértices consecutivos |
| **Relleno** | Algoritmo de **scanline** con intersecciones ordenadas por pares |

El dibujo final tiene 5 polígonos: una estrella, un rombo, un triángulo y una
tetera. El **polígono 5 es un agujero** dentro de la tetera — se logra
rellenándolo con el color de fondo, y trazando los contornos *después* de todos
los rellenos para que el agujero no borre el borde del polígono 4.

## Estructura

```
src/
├── main.rs          Define los 5 polígonos y arma la escena
├── framebuffer.rs   Buffer de pixeles + exportadores a BMP y PNG
├── point.rs         point()  -> set_pixel
├── line.rs          line()   -> point()
└── polygon.rs       polygon() y fill_polygon() -> line()
```

Salidas: `out.bmp` y `out.png` (misma imagen, 800x500).

> El eje Y crece hacia arriba: `y = 0` es la fila de abajo (coordenadas
> matemáticas). El exportador PNG invierte la Y explícitamente; el BMP no lo
> necesita porque el formato ya almacena las filas de abajo hacia arriba.

## Requisitos

- **Rust 1.85+** (el proyecto usa edition 2024)
- **CMake** — `raylib-sys` compila raylib desde código C
- **Compilador C/C++** — MSVC Build Tools en Windows
- **LLVM / libclang** — lo necesita `bindgen` para generar los bindings de raylib

En Windows, con [winget](https://learn.microsoft.com/windows/package-manager/):

```powershell
winget install --id Kitware.CMake
winget install --id LLVM.LLVM
winget install --id Microsoft.VisualStudio.2022.BuildTools --override "--quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

Si `bindgen` no encuentra libclang, apúntalo explícitamente:

```powershell
$env:LIBCLANG_PATH = 'C:\Program Files\LLVM\bin'
```

> Tras instalar, abre una terminal **nueva** para que tome el PATH actualizado.

## Uso

```bash
cargo run
```

Genera `out.bmp` y `out.png` en la raíz del proyecto.

## Desarrollo por ramas

El laboratorio se construyó de forma incremental, una rama por primitiva:

```
feature/dots      -> point.rs
feature/lines     -> line.rs (Bresenham)
feature/polygons  -> polygon()
feature/fill      -> fill_polygon() (scanline)
Polygon-1 .. Polygon-4  -> cada polígono del dibujo
feature/final     -> los 5 polígonos juntos + export a BMP
```

Cada rama entra a `main` mediante un merge; `main` no recibe commits directos.

## Dependencias

- [`raylib`](https://crates.io/crates/raylib) — solo por los tipos `Vector2` y `Color`
- [`image`](https://crates.io/crates/image) — para el export a PNG
