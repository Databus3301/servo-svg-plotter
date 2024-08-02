#!/bin/bash

inkscape $1 --batch-process --actions="select-all;object-to-path;vacuum-defs;export-filename:out.svg;export-plain-svg;export-overwrite;export-do"
