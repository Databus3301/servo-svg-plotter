# servo-svg-plotter

#### Goal

> Plotting (SVG) vectorgraphics with a self designed robot. <br><br>
> Specifically bezier curves (which all outlines can be converted to) <br>
> using their curvature and vector decomposition <br>
> to spin two connected threaded shafts using servos accordingly
> and have a pen trace their combined movement

#### Documentation

The motor inputs are derived from an arbitrary svg (without _arcs_).<br>
To accomplish this the project includes an svg-path interpreter in Rust.<br>
All path data is represented in Bezier curve structs and saved alongside other metadata (width, height) in an SVG struct.

[inkscape](https://github.com/inkscape/inkscape) is used to convert all objects to paths and simplify them using [this sh](https://github.com/Databus3301/servo-svg-plotter/blob/main/src/convert_svg.sh)
<br>
<br>

check out [doc](https://github.com/Databus3301/servo-svg-plotter/tree/main/doc) for non code related documentation


<br><br><br><br>
<sub>This project originated as part of the <a href="https://code.design/">Code+Design Camp Bochum</a></sub>
