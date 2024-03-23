---
## Front matter
lang: ru-RU
title: Лабораторная работа 7
author:
  - Генералов Даниил, 1032212280
institute:
  - Российский университет дружбы народов, Москва, Россия
date: 2024

## i18n babel
babel-lang: russian
babel-otherlangs: english

## Formatting pdf
toc: false
toc-title: Содержание
slide_level: 2
aspectratio: 169
section-titles: true
theme: metropolis
header-includes:
 - \metroset{progressbar=frametitle,sectionpage=progressbar,numbering=fraction}
 - '\makeatletter'
 - '\beamer@ignorenonframefalse'
 - '\makeatother'
---

## Задача

> Постройте график распространения рекламы, математическая модель которой описывается следующим уравнением:
> 
> 1. dn/dt = (0.7 + 0.000012n(t))(N-n(t))
> 2. dn/dt = (0.00003 + 0.5n(t))(N-n(t))
> 3. dn/dt = (0.57sin(t) + 0.38cos(13t)n(t))(N-n(t))
> 
> При этом объем аудитории N=1420, в начальный момент о товаре знает 12 человек. Для случая 2 определите в какой момент времени скорость распространения рекламы будет иметь максимальное значение.

## Выполнение: Julia

![Julia: уравнения](../report/image/1.png){#fig:001 width=70%}

## Выполнение: Julia

![Julia: задача 1](../report/image/2.png){#fig:002 width=70%}

## Выполнение: Julia

![Julia: задача 2](../report/image/3.png){#fig:003 width=70%}

## Выполнение: Julia

![Julia: задача 3](../report/image/3.5.png){#fig:0035 width=70%}


## Выполнение: OpenModelica
![OpenModelica: задача 1](../report/image/4.png){#fig:004 width=70%}

## Выполнение: OpenModelica

![OpenModelica: задача 1 решение](../report/image/5.png){#fig:005 width=70%}

## Выполнение: OpenModelica

![OpenModelica: задача 2](../report/image/6.png){#fig:006 width=70%}

## Выполнение: OpenModelica

![OpenModelica: задача 2 решение](../report/image/7.png){#fig:007 width=70%}

## Выполнение: OpenModelica

![OpenModelica: задача 3](../report/image/7.5.png){#fig:0075 width=70%}

## Выполнение: OpenModelica

![OpenModelica: задача 3 решение](../report/image/8.png){#fig:008 width=70%}

## Выводы

Мы смогли получить ответы на задачу в OpenModelica и Julia,
и определить источник проблемы в решении на Julia.
