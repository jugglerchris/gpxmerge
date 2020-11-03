# gpxmerge
Simple tool to merge two GPX tracks together.

Usage:
    gpxmerge [--rename='My GPX Name'] --output=output.gpx foo.gpx bar.gpx baz.gpx

Appends any tracks in bar.gpx, baz.gpx to those in foo.gpx, and writes out as
the output.gpx; the metadata comes from the first specified file but with an
updated name.

To do:
* Look into whether any other components should be merged as well as the tracks.
* Configurability for other usecases.

Pull requests, bug reports, feature requests and questions welcome.