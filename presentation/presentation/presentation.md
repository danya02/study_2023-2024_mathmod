---
## Front matter
lang: ru-RU
title: Модель заражения SIR
author:
  - Генералов Д. М. 1032212280
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
  - \usepackage{indentfirst}
  - \usepackage{float} # keep figures where there are in the text
  - \floatplacement{figure}{H} # keep figures where there are in the text
---

## Компартментальные модели

![](./image/1920px-Diagram_of_SIR_epidemic_model_states_and_transition_rates.svg.png)

## SIR-модель в дифф.ур.

$$
  \begin{cases}
    \frac{dS}{dt} = -\frac{\beta I S}{N} \\
    \frac{dI}{dt} = \frac{\beta I S}{N} - \gamma I \\
    \frac{dR}{dt} = \gamma I
  \end{cases}
$$

где:

- $\beta$ = среднее число контактов за единицу времени * вероятность передачи одним контактом
- $\gamma = \frac{1}{D}$, где $D$ -- время течения заболевания

## SIR-модель по графику

![](./image/SIR_trajectory.png)

## $R_0$ -- показатель заразности болезни

![](./image/R_Naught_Ebola_and_Flu_Diagram.svg.png)

$$R_0 = \frac{\beta}{\gamma}$$

SARS-CoV-2: $2.9 \rightarrow 4.0 \rightarrow 5.1 \rightarrow 9.5$

## Количество выздоровевших меняет скорость распространения

![](./image/susceptibles.png)

$$\frac{dS}{dt} = -\frac{\beta I S}{N}$$


## $R$ -- показатель распространения болезни

![](./image/r2.png)

## SIRS/SEIRS-модели

Иммунитет может быть потерян за время

![](./image/Screenshot 2024-02-23 at 12-42-38 What Happens Next COVID-19.png)

## Применения модели: пик

![](./image/Screenshot 2024-02-23 at 13-22-50 File 20200403 Flatten the curve animated GIF.gif - Wikimedia Commons.png)

![](./image/Screenshot 2024-02-23 at 13-23-21 File 20200403 Flatten the curve animated GIF.gif - Wikimedia Commons.png)

## Применения модели: хвост

![](./image/Screenshot 2024-02-23 at 13-27-13 3Blue1Brown.png)

## Применения модели: финальное количество R

![](./image/Screenshot 2024-02-23 at 13-31-41 3Blue1Brown.png)

# Спасибо за внимание