from lxml import etree
from svg.path import parse_path
from svg.path.path import CubicBezier
import sys
import os

def convert_path_to_beziers(d):
    path = parse_path(d)
    bezier_path = []
    for segment in path:
        if isinstance(segment, CubicBezier):
            bezier_path.append(segment)
        else:
            bezier_path.extend(segment.to_cubic())
    return bezier_path

def convert_svg(input_file, output_file):
    tree = etree.parse(input_file)
    root = tree.getroot()

    for path in root.findall('.//{http://www.w3.org/2000/svg}path'):
        d = path.get('d')
        if d:
            bezier_path = convert_path_to_beziers(d)
            path.set('d', ' '.join([str(p) for p in bezier_path]))

    tree.write(output_file, pretty_print=True, xml_declaration=True, encoding='UTF-8')
    print(f"Converted SVG saved as {output_file}")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python convert_svg_to_beziers.py input.svg output.svg")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    if not os.path.isfile(input_file):
        print(f"Input file {input_file} does not exist.")
        sys.exit(1)

    convert_svg(input_file, output_file)