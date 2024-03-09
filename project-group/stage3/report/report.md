---
## Front matter
title: "Моделирование образования планетной системы"
subtitle: "Этап 3"
author: "Матюхин Г. В. и Генералов Д. М."

## Generic otions
lang: ru-RU
toc-title: "Содержание"

## Bibliography
bibliography: bib/cite.bib
csl: pandoc/csl/gost-r-7-0-5-2008-numeric.csl

## Pdf output format
toc: true # Table of contents
toc-depth: 2
lof: true # List of figures
lot: true # List of tables
fontsize: 12pt
linestretch: 1.5
papersize: a4
documentclass: scrreprt
## I18n polyglossia
polyglossia-lang:
  name: russian
  options:
	- spelling=modern
	- babelshorthands=true
polyglossia-otherlangs:
  name: english
## I18n babel
babel-lang: russian
babel-otherlangs: english
## Fonts
mainfont: PT Serif
romanfont: PT Serif
sansfont: PT Sans
monofont: PT Mono
mainfontoptions: Ligatures=TeX
romanfontoptions: Ligatures=TeX
sansfontoptions: Ligatures=TeX,Scale=MatchLowercase
monofontoptions: Scale=MatchLowercase,Scale=0.9
## Biblatex
biblatex: true
biblio-style: "gost-numeric"
biblatexoptions:
  - parentracker=true
  - backend=biber
  - hyperref=auto
  - language=auto
  - autolang=other*
  - citestyle=gost-numeric
## Pandoc-crossref LaTeX customization
figureTitle: "Рис."
tableTitle: "Таблица"
listingTitle: "Листинг"
lofTitle: "Список иллюстраций"
lotTitle: "Список таблиц"
lolTitle: "Листинги"
## Misc options
indent: true
header-includes:
  - \usepackage{indentfirst}
  - \usepackage{float} # keep figures where there are in the text
  - \floatplacement{figure}{H} # keep figures where there are in the text
  - \usepackage{fvextra}
  - \DefineVerbatimEnvironment{Highlighting}{Verbatim}{breaklines,commandchars=\\\{\}}
---

# Цель работы

Провести моделирование одного из этапов эволюции Вселенной -- образование некой "солнечной" системы из межзвездного газа.

# Выполнение

Весь код, который был написан в этой работе, можно увидеть по ссылке: https://github.com/danya02/study_2023-2024_mathmod/tree/master/project-group/stage3

## Julia

Для каждой частицы мы храним актуальные для симуляции свойства: ее позицию, вектор скорости, массу и радиус.

```julia
mutable struct Particle
    position
    velocity
    mass::Float64
    radius::Float64
end
```

Для того, чтобы посчитать одну симуляцию, следует:
1. Сгенерировать частицы
2. Много раз двигать частицы под воздейстием сил

```julia
particles = [get_random_particle(10, 1) for _ in 1:1000] #1
for i in 1:100
    apply_particle_forces(particles, dt) #2
end
```

### Генерация частиц

Все параметры частиц рандомизированы при их генерации.
Частицы располагаются равномерно на расстоянии до `radius_max` от центра координат.

```julia
function get_random_particle(radius_max, angular_speed)::Particle
    radius = sqrt(Random.rand()) * radius_max
    angle = Random.rand() * 2 * pi
    x = radius * cos(angle)
    y = radius * sin(angle)
    vx = -y * angular_speed * (radius_max / radius)^(3/2)
    vy = x * angular_speed * (radius_max / radius)^(3/2)
    Particle([x,y], [vx,vy], Random.rand() * max_mass, Random.rand() * max_radius)
end
```

### Движение под воздейстием сил

Для каждой частицы мы вычисляем силы притяжения от всех других частиц. Используя их сумму мы изменяем вектор скорости.

```julia
for src_part in particles
    force = [0.0, 0.0]
    for dst_part in particles
        if src_part === dst_part
            continue
        end
        extra_force = normalize(src_part.position - dst_part.position) * universal_gravitation_const * (src_part.mass * dst_part.mass) / magnitude(src_part.position - dst_part.position)
        force += extra_force
    end
end
```

Далее мы корректируем вектор скорости частицы и изменяем ее позицию.

```julia
src_part.velocity += force * dt
src_part.position += src_part.velocity * dt
```

### Результаты

Хотя приведенный выше код на Julia выполняет основные функции симуляции, он очень медленный. Расчет отталкиваний и слипаний частиц, а также визуализация симуляции увеличат время выполнения еще в несколько раз. Это из-за того, что Julia -- динамически типизированный язык, и каждая переменная должна содержать информацию о своем типе; также, любое использование переменной, для которой в контексте использования неизвестен тип, приводит к дополнительной задержке на поиск правильной реализации функции для этого типа [@juliaperf].

Например, код как написан здесь -- без соединения частиц и без каких-либо дополнительных сил, только с подсчетом гравитационного взаимодействия между 1000 частицами в течении 100 шагов -- занимает 80 секунд на доступном нам компьютере. Если поднять количество частиц до 5000, то даже 50 шагов симуляции будут выполняться 14 минут и 47 секунд. И это очень маленькое количество: для того, чтобы получить разумные выводы из симуляции, нужно на порядок больше частиц и гораздо больше шагов времени. 

К счастью, в Julia есть хорошие возможности для взаимодействия с кодом, скомпилированным на других языках вроде C или Fortran [@juliac];
большинство библиотек на Julia используют для самых сложных вычислений нативный код.
Поэтому нами было принято решение использовать комбинацию Rust и Julia для достижения лучшей произовдительности программы.

Это решение весьма эффективно: с переносом самых трудных задач в нативный код,
1000 частиц в течении 100 шагов симулируются 5.6 секунды,
а 5000 частиц и 50 шагов -- в течении 17.6 секунд (и из них 10.8 занимает сам подсчет, а остальное время -- постобработка). Это ускорение в 14 и 50 раз соответственно. 

## Rust && Julia

Код со стороны Julia выглядит примерно так же.

Структура частицы повторяется. Основные различия -- теперь она полностью инстанциирована вариантами типов своих значений,
а также имеет параметр `id` -- это поможет определить частицы, когда их порядок меняется в списках.
Структуры объявляются так же на другой стороне FFI.

```julia
struct Vec2
    x::Float64
    y::Float64
end

struct Particle
    id::UInt64
    position::Vec2
    velocity::Vec2
    mass::Float64
    radius::Float64
end
```

При генерировании частиц мы задаем все значения случайно, но теперь также связываем плотность и объем. Это позволяет делать более реалистичное распределение радиусов против масс частиц, что приведет к более реалистичному поведению системы.

```julia
function get_random_particle(radius_max, angular_speed, i)::Particle
    radius = sqrt(Random.rand()) * radius_max
    angle = Random.rand() * 2 * pi
    x = radius * cos(angle)
    y = radius * sin(angle)
    vx = -y * angular_speed * (radius_max / radius)^(3/2)
    vy = x * angular_speed * (radius_max / radius)^(3/2)
    mass = Random.rand() * max_part_mass
    density = Random.rand() * (max_density - min_density) + min_density
    volume = mass / density
    radius = cbrt((3/4pi) * volume)
    Particle(i, Vec2(x,y), Vec2(vx,vy), mass, radius)
end
```

Частицы также создаются, а затем симулируются фиксированное количество шагов по времени.
Это создает таблицу, где каждая строка -- это один шаг симуляции,
а внутри этой строки текущие состояния частиц.

```julia
particles = [get_random_particle(200, 2, i) for i=0:1000]

frames = @time perform_timesteps(particles, 1000)
```

Однако, в отличии от предыдущей попытки, теперь эти вычисления делаются в нативном машинном коде.
Для этого нужно подготовить память:
мы создаем эту таблицу,
заполняем ее конкретным значением (для удобства отладки),
а затем используем функционал Julia для вызова нативного кода,
передавая туда указатели на таблицу и ее размеры.
Затем можно сразу сохранить результаты, или можно продолжить их обработку.

```julia
function perform_timesteps(particles::Vector{Particle}, steps)
    particle_count = Csize_t(length(particles))
    step_count = Csize_t(steps)
    step_array = Vector{Vector{Particle}}(undef, steps);
    
    for i in 1:steps
        step_array[i] = Vector{Particle}(undef, length(particles));
    end
    step_array[1] = particles;
    for i in 2:steps
        for j in 1:length(particles)
            step_array[i][j] = get_uninit_particle(j)
        end
    end

    sym = Libdl.dlsym(LIB, :perform_timesteps)
    @ccall $sym(step_array::Ptr{Ptr{Particle}}, particle_count::Csize_t, step_count::Csize_t)::Cvoid

    # # save simulation results
    # f = open(io -> write(io, JSON.json(step_array)), "output.json", write=true)
    step_array
end
```

### Симуляция

В нативном коде мы сначала приводим переданные указатели к стандартным ссылкам на списки,
а затем берем каждую строчку как входные данные, а в следующую записываем результаты шага симуляции.
Мы также храним количество частиц, которые активно участвуют в симуляции,
и убеждаемся, что все они находятся в начале списка:
это позволяет игнорировать их на последующих итерациях, потому что они больше никогда не станут активными.
Мы также выводим строчку с информацией каждый раз, когда какие-то частицы становятся неактивными.

```rust
#[no_mangle]
pub extern "C" fn perform_timesteps(
    data: *mut *mut Particle,
    particle_count: usize,
    step_count: usize,
) {
    // data points to an array of pointers, the len of the array is step_count.
    // each of the pointers points to an array of Particles, the len of each of them is particle_count.
    // The arrays are modified in place.
    let particle_ptr_list = unsafe { std::slice::from_raw_parts(data, step_count) };
    let mut particle_slice_list = Vec::with_capacity(step_count);
    for ptr in particle_ptr_list {
        let data_at_ptr = *ptr;
        let slice = unsafe { std::slice::from_raw_parts_mut(data_at_ptr, particle_count) };
        particle_slice_list.push(slice);
    }

    let dt = 0.001;
    let mut living_particles = particle_count;
    for src_count in 0..step_count - 1 {
        let [src, dst] = &mut particle_slice_list[src_count..=src_count + 1] else {
            unreachable!()
        };

        dst.copy_from_slice(src);
        if living_particles == 0 {
            continue;
        }

        dst[..living_particles].chunks_mut(10).for_each(|dst| {
            apply_particle_forces(&src[..living_particles], dst, dt);
        });

        let glued_particles = run_glue(dst);
        if glued_particles > 0 {
            sort_zeroed(dst);
            living_particles -= glued_particles;
            println!("step {src_count}: {living_particles} living");
        }
    }
}
```

Вычисления влияния гравитационных сил от других частиц остается таким-же как и при использовании только Julia, но происходит уже на нативной стороне.

```rust
pub fn apply_particle_forces(
    all_src_particles: &[Particle],
    my_particles: &mut [Particle],
    dt: f64,
) {
    for src in my_particles.iter_mut() {
        if src.mass == 0.0 {
            continue;
        }
        let mut force = Vec2::new(0.0, 0.0);
        for dst in all_src_particles.iter() {
            if src == dst {
                continue;
            }
            let extra_force = (dst.position - src.position).normalize()
                * UNIVERSAL_GRAVITATION
                * ((src.mass * dst.mass) / (src.position - dst.position).magnitude());
            force += extra_force;
        }
        src.velocity += force * dt;
        src.position += src.velocity * dt;
    }
}
```

Частицы должны становиться неактивными, когда они слипаются с другими частицами. Они не просто удаляются, потому что таблица имеет фиксированный размер:
она должна хранить максимальное число частиц, хотя оно будет только уменьшаться относительно первого шага симуляции.
Поэтому, когда две частицы слипаются в одну, одна из них модифицируется, становясь комбинацией обоих, а вторая получает радиус и массу равными нулю:
как результат, она больше не участвует в гравитационных взаимодействиях.

```rust
pub fn run_glue(dst_particles: &mut [Particle]) -> usize {
    let mut glued_particles = 0;
    for idx in 1..dst_particles.len() {
        let (first, rest) = dst_particles.split_at_mut(idx);
        let first = first.last_mut().unwrap();
        for other in rest {
            glued_particles += if other.maybe_glue_to_other(first) {
                1
            } else {
                0
            };
        }
    }
    glued_particles
}

impl Particle {
    pub fn is_zeroed(&self) -> bool {
        self.mass == 0.0 || self.radius == 0.0
    }

    pub fn glue_to_other(&mut self, other: &mut Self) {
        self.position = ((self.position * self.mass) + (other.position * other.mass))
            / (self.mass + other.mass);
        self.velocity = ((self.velocity * self.mass) + (other.velocity * other.mass))
            / (self.mass + other.mass);
        self.mass += other.mass;
        self.radius = f64::cbrt(self.radius.powi(3) + other.radius.powi(3));
        other.velocity = Vec2::new(0.0, 0.0);
        other.radius = 0.0;
        other.mass = 0.0;
    }

    pub fn maybe_glue_to_other(&mut self, other: &mut Self) -> bool {
        if self.is_zeroed() || other.is_zeroed() {
            return false;
        }
        if (self.position - other.position).magnitude() < self.radius + other.radius {
            self.glue_to_other(other);
            return true;
        }
        false
    }
}
```

Из-за своего фокуса на производительность и корректность, Rust имеет библиотеки для работы с параллельными вычислениями,
где валидность операций статически доказывается: если ваш код компилируется, то он будет работать правильно (такое свойство иногда называют _fearless concurrency_, бесстрашный параллелизм) [@fearless].
Это свойство позволяет легко ускорить работу основного цикла гравитационного взаимодействия:
мы запускаем несколько потоков, каждый из которых обновляет только небольшой набор всех частиц
на основе общего предыдущего состояния.
Это делается очень легко посредством библиотеки Rayon [@rayon]:
она заменяет стандартные итераторы параллельными версиями, которые внутренне распределяются на много потоков.

```rust
pub fn apply_particle_forces(
    all_src_particles: &[Particle],
    my_particles: &mut [Particle],
    dt: f64,
) {
    for src in my_particles.iter_mut() {
        if src.mass == 0.0 {
            continue;
        }
        let mut force = Vec2::new(0.0, 0.0);
        for dst in all_src_particles.iter() {
            if src == dst {
                continue;
            }
            let extra_force = (dst.position - src.position).normalize()
                * UNIVERSAL_GRAVITATION
                * ((src.mass * dst.mass) / (src.position - dst.position).magnitude());
            force += extra_force;
        }
        src.velocity += force * dt;
        src.position += src.velocity * dt;
    }
}

pub fn run_glue(dst_particles: &mut [Particle]) -> usize {
    let mut glued_particles = 0;
    for idx in 1..dst_particles.len() {
        let (first, rest) = dst_particles.split_at_mut(idx);
        let first = first.last_mut().unwrap();
        for other in rest {
            glued_particles += if other.maybe_glue_to_other(first) { 1 } else { 0 };
        }
    }
    glued_particles
}
```

После того, как мы вернулись из функции подсчета динамики частиц,
мы могли бы сразу визуализировать это.
Однако, из-за технических деталей,
Pygame сложно использовать для визуализации,
а в Blender создание большого количества ключевых кадров (keyframe) экспоненциально замедляет создание новых,
из-за чего визуализация будет проходить дольше, чем сама симуляция.
Поэтому,  после этого мы должны сократить количество точек данных с помощью алгоритма децимации Рамера — Дугласа — Пекера [@RAMER1972244].

```rust
fn rdp(points: &[TimeSeriesPoint], epsilon: f64, result: &mut Vec<TimeSeriesPoint>) {
    let n = points.len(); let mut max_dist = 0.0; let mut index = 0;
    for i in 1..n - 1 {
        let dist = perpendicular_distance(&points[i], &points[0], &points[n - 1]);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }
    if max_dist > epsilon {
        rdp(&points[0..=index], epsilon, result);
        rdp(&points[index..n], epsilon, result);
    } else {
        result.push(points[n - 1]);
    }
}
```

## Выводы

На этом этапе проекта мы смогли сделать программу, которая считает динамику системы частиц достаточно быстро, чтобы можно было попробовать различные варианты начальных конфигураций и сделать анализ свойств солнечной системы. Это потребует отслеживания метрик симуляции (вроде общей энергии системы) и визуализаций с помощью Blender. 

# Список литературы{.unnumbered}

::: {#refs}
:::
