---
## Front matter
lang: ru-RU
title: Лабораторная работа 4
author:
  - Генералов Даниил 1032212280
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

> Постройте фазовый портрет гармонического осциллятора и решение уравнения
гармонического осциллятора для следующих случаев
> 1. Колебания гармонического осциллятора без затуханий и без действий внешней силы x'' + 1.7x = 0
> 2. Колебания гармонического осциллятора c затуханием и без действий внешней силы: x'' + 1.7x' + 1.7x = 0
> 3. Колебания гармонического осциллятора c затуханием и под действием внешней силы: x'' + 2x' + 1.7x = 0.7cos(2.7t)
> На интервале t `\in` [0; 59] (шаг 0.05) с начальными условиями x_0=1.7, y_0=-0.2





## Выполнение: Julia

![Julia: начальные условия](../report/image/1.png){#fig:001 width=70%}

## Выполнение: Julia
![Julia: простой маятник](../report/image/2.png){#fig:002 width=70%}

## Выполнение: Julia
![Julia: простой маятник диаграмы](../report/image/3.png){#fig:003 width=70%}

## Выполнение: Julia
![Julia: затухающий маятник](../report/image/4.png){#fig:004 width=70%}

## Выполнение: Julia
![Julia: затухающий маятник диаграмы](../report/image/5.png){#fig:005 width=70%}

## Выполнение: Julia
![Julia: затухающий маятник с дополнительной энергией](../report/image/6.png){#fig:006 width=70%}

## Выполнение: Julia
![Julia: затухающий маятник диаграмы с дополнительной энергией](../report/image/7.png){#fig:007 width=70%}

## Выполнение: OpenModelica
![OpenModelica: незатухающий маятник](../report/image/8.png){#fig:008 width=70%}

## Выполнение: OpenModelica
![OpenModelica: незатухающий маятник график](../report/image/9.png){#fig:009 width=70%}

## Выполнение: OpenModelica
![OpenModelica: незатухающий маятник фазовая диаграма](../report/image/10.png){#fig:010 width=70%}

## Выполнение: OpenModelica
![OpenModelica: затухающий маятник](../report/image/11.png){#fig:011 width=70%}

## Выполнение: OpenModelica
![OpenModelica: затухающий маятник график](../report/image/12.png){#fig:012 width=70%}

## Выполнение: OpenModelica
![OpenModelica: затухающий маятник фазовая диаграма](../report/image/13.png){#fig:013 width=70%}

## Выполнение: OpenModelica
![OpenModelica: затухающий маятник с дополнительной энергией](../report/image/14.png){#fig:014 width=70%}

## Выполнение: OpenModelica
![OpenModelica: затухающий маятник график с дополнительной энергией](../report/image/15.png){#fig:015 width=70%}

## Выполнение: OpenModelica
![OpenModelica: затухающий маятник фазовая диаграма с дополнительной энергией](../report/image/16.png){#fig:016 width=70%}


## Вывод

> Мы смогли успешно симулировать поведение маятника без затухания, с ним и с дополнительной энергией.
