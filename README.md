# cli_base_converter
A simple tool written in rust to convert numbers from one base to another. Accepts current base as argument but will auto-detect base if that is not provided

## Fields
dest_base: String
The base to convert to

input: String
The number to convert

org_base: Option<String>
The base of the input number/base to convert from

## Input
The program accepts input in two formats
1. \<dest_base\> \<input\> \<org_base\>
or
2. \<dest_base\> \<input\>

org_base is calculated if not provided
