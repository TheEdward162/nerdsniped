#!/usr/bin/env python3

import sys
import pandas
import plotly.express as px

def fmt_delta(delta):
	if delta.seconds == 0:
		ms = delta.microseconds / 1000
		return f"{ms:.3f}ms"
	
	s = (delta.seconds % 60) + delta.microseconds / 1_000_000
	if delta.seconds < 60:
		return f"{s:.2f}s"
	
	m = delta.seconds // 60
	return f"{m}m{s:.0f}s"

data = pandas.read_csv(sys.argv[1], converters = { "min": lambda x: pandas.to_timedelta(x + "s") })
data["history"] = data.apply(lambda r: r.name - max(data[data.command == r.command].index), axis = 1)
data["value_fmt"] = data.apply(lambda r: fmt_delta(r["min"]), axis = 1)

fig = px.line(data, x = "history", y = "min", color = "command", title = "Min times", text = "value_fmt", markers = True)
fig.update_layout(
	plot_bgcolor = "white",
    xaxis = dict(
        showline = True,
        showgrid = False,
		zeroline = False,
        showticklabels = False
    ),
    yaxis = dict(
        showgrid = True,
		gridcolor = "lightgrey",
        zeroline = False,
        showline = False,
        showticklabels = False
    )
)
fig.update_traces(textposition = "top center")
fig.show()
