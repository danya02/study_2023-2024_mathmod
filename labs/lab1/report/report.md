---
## Front matter
title: "Лабораторная работа 1"
subtitle: ""
author: "Генералов Д. М. 1032212280"

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
---

# Цель работы

Настройка рабочего окружения для выполнения последующих лабораторных работ.


# Выполнение лабораторной работы

Сначала я зашел на GitHub на страницу template для создания рабочего репозитория (рис. @fig:001).

![GitHub template](image/1.png){#fig:001 width=70%}

После этого я создал репозиторий на основе него (рис. @fig:002).

![Использование GitHub template](image/2.png){#fig:002 width=70%}

Затем я зашел в папку, в которой я храню университетские проекты, и склонировал новый репозиторий туда (рис. @fig:003).

![Клонирование репозитория](image/3.png){#fig:003 width=70%}

Используя предоставленный Makefile, я скачал субмодули для презентации и отчета,
а затем размножил их в соответствии с необходимой структурой (рис. @fig:004).

![Создание нужной структуры директорий](image/4.png){#fig:004 width=70%}

Я также удалил лишние файлы в корне директории, но по результатам у меня были созданы папки `labs`, `presentation` и `project-group` (рис. @fig:005).

![Измененные файлы](image/5.png){#fig:005 width=70%}

После того, как это было сделано, я написал этот отчет,
презентацию, и другие необходимые файлы,
и создал и отправил на сервер коммит с ними.

# Выводы

Используя подготовленное окружение, можно выполнять дальнейшие лабораторные работы.
