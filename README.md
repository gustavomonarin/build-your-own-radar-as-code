# Build your own radar as code

Thoughtworks tech radar is a great way to spread knowledge, visualize entropy and align teams.

Several people advocate that companies to implement their own internal radar which can be an easy task provided the
simple Thoughtworks build-your-own-radar website which is feeded by a google spreadsheet. Several companies have
implemented their own code to build their radar as wel.

However, all of them apparently focus on the visualization/creation process, not in a process of sharing the knowledge
/building the radar in an inclusive and iterative way.

Here is some experimentation of having a github as an easily editable source of the radar and use all the beaut of pull
requests, pipelines, snapshots, releases and of course github pages ;).

For some personal extra thoughts about knowledge sharing and previous experience checkout this [blog post]()

# build-your-own-radar-as-code

# Blip format

Blips are described as markdown files in order to be human-readable and easy maintenance within the source code.

To provide extra metadata, for instance quadrant and rings, the
pattern [front matter](https://jekyllrb.com/docs/front-matter/) used by jenkyl to static pages is used. The front matter
pattern simply consist of a small section at the beginning of the file with the required metadata in the yaml format
followed by a body containing the full text [jenkyll example](https://raw.githubusercontent.
com/jekyll/jekyll/master/docs/_
posts/2020-12-14-jekyll-4-2-0-released.markdown).

# TODO
* 
* Connect the new blip document with the main
* Make binary easily installable
* Typed errors?
* Markdown to html converter
* Move the tests to a different area
* Create an inner structure for the blip document front matter
* Reduce of the String objects by slices and obect life tiems?
