# Proyecto: Simulación de Modelo Depredador–Presa con Rust y Macroquad

## 🎯 Objetivos

1. **General**
   - Desarrollar un simulador en Rust que modele la dinámica depredador–presas,
     aplicando conceptos de programación estructurada, polimorfismo con traits,
     y manejo de estado.

2. **Específicos**
   - Implementar un ecosistema simplificado con varias especies de presas y un único depredador.
   - Definir las reglas biológicas y probabilísticas:
     - Crecimiento (curva de Gompertz).
     - Reproducción (edad mínima, probabilidades de crías, sexo, enfermedad).
     - Estrategia de caza del depredador (edad de sacrificio, selección de presa).
   - Incorporar la reserva de alimento del depredador con niveles mínimo y óptimo de consumo.
   - Permitir que el sistema evolucione en ciclos diarios hasta alcanzar o aproximarse a un equilibrio poblacional.
   - Visualizar la simulación en tiempo real con **macroquad** mostrando la evolución de poblaciones y estados.
   - Documentar la arquitectura del software mediante diagramas UML y justificación de diseño.

---

## 🛠️ Metodología

1. **Revisión conceptual**
   - Estudiar la dinámica depredador–presa y la curva de crecimiento de Gompertz.
   - Revisar las probabilidades asociadas a natalidad, enfermedad y mortalidad.

2. **Diseño del sistema**
   - Elaborar un **diagrama de contexto** para definir límites y actores externos.
   - Diseñar un **diagrama de clases UML** con `traits` y `structs` principales
     (`Organismo`, `Presa`, `Depredador`, `Mundo`).
   - Crear un **diagrama de secuencia/flujo de datos** para ilustrar un ciclo de simulación.
   - Justificar las decisiones de diseño (modularidad, uso de traits, etc.).

3. **Implementación en Rust**
   - Configurar proyecto con **Cargo**.
   - Definir módulos y estructuras de datos.
   - Programar las reglas de evolución diaria del sistema.
   - Incorporar generación aleatoria controlada (RNG con semilla reproducible).

4. **Visualización con Macroquad**
   - Desarrollar representación gráfica 2D del mundo simulado.
   - Mostrar indicadores clave: día actual, número de presas por especie, estado del depredador y reservas de comida.
   - Mantener claridad y usabilidad en la visualización.

5. **Pruebas y ajuste**
   - Simular diferentes escenarios iniciales y observar comportamientos.
   - Verificar estabilidad, equilibrio o extinción de poblaciones.
   - Ajustar parámetros hasta obtener resultados coherentes.

6. **Documentación final**
   - Redactar el documento con diagramas y explicaciones.
   - Comentar adecuadamente el código.
   - Entregar repositorio Git o carpeta comprimida con:
     - Código fuente (Cargo).
     - Documento de diseño.
     - Instrucciones de uso.
