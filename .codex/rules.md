#  testing rules

- Los tests del backend Tauri deben vivir bajo `tests/backend`.
- Los tests del frontend Tauri deben vivir bajo `tests/frontend`.
- Antes de modificar codigo o tests en cada interaccion se debe leer este archivo de reglas.
- Se permiten harnesses sin tests en `tests/backend.rs` y `tests/frontend.rs` solo para cargar automaticamente los archivos de `tests/backend` y `tests/frontend`.
- Los archivos nuevos bajo `tests/backend` y `tests/frontend` deben ser descubiertos automaticamente por los harnesses, sin agregar nuevos bloques `[[test]]` en `Cargo.toml`.
- Debe existir un archivo de test por cada archivo backend testeado y debe respetar el mismo nombre base.
- Debe existir un archivo de test por cada archivo frontend testeado y debe respetar el mismo nombre base.
- Las funciones de test deben usar la nomenclatura `GivenXXXX_WhenYYYY_ThenZZZZ_ShouldMMMM`.
- Se deben usar mocks o doubles cuando sea posible para simular capas inferiores y aislar la unidad bajo prueba.
- El frontend debe usar mocks para simular las llamadas al backend y evitar dependencias externas en los tests de frontend.
- El coverage minimo obligatorio para backend es 80%.
- El objetivo preferido para backend es superar 90% de coverage.
- Al terminar cualquier modificacion o adicion de codigo se debe ejecutar de nuevo los tests y verificar que el coverage se mantiene o mejora, especialmente para las partes del codigo que fueron modificadas o añadidas.
