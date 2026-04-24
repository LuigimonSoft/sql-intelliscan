# Backend testing rules

- Los tests del backend Tauri deben vivir bajo `tests/backend`.
- Debe existir un archivo de test por cada archivo backend testeado y debe respetar el mismo nombre base.
- Las funciones de test deben usar la nomenclatura `GivenXXXX_WhenYYYY_ThenZZZZ_ShouldMMMM`.
- Se deben usar mocks o doubles cuando sea posible para simular capas inferiores y aislar la unidad bajo prueba.
- El coverage minimo obligatorio para backend es 80%.
- El objetivo preferido para backend es superar 90% de coverage.
