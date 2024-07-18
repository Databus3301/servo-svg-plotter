#!/bin/bash

inkscape $1 --batch-process --actions="select-all;object-to-path;export-filename:out.svg;export-do"