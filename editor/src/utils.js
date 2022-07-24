export function calc_route_multipoly(stops, route_stops) {
    let segments = [];

    let current_segment = [];
    for (let i = 0; i < route_stops.length - 1; i++) {
        let firstStopId = route_stops[i];
        let secondStopId = route_stops[i + 1];
        if (stops[firstStopId].lon && stops[secondStopId].lon) {
            if (current_segment.length === 0) {
                current_segment.push([stops[firstStopId].lat, stops[firstStopId].lon]);
            }
            current_segment.push([stops[secondStopId].lat, stops[secondStopId].lon]);
        } else {
            if (current_segment.length !== 0) {
                segments.push(current_segment);
                current_segment = [];
            }
        }
    }

    if (current_segment.length !== 0) {
        segments.push(current_segment);
    }

    return segments;
}


function randomInteger(max) {
    return Math.floor(Math.random() * (max + 1));
}

function randomRgbColor() {
    let r = randomInteger(255);
    let g = randomInteger(255);
    let b = randomInteger(255);
    return [r, g, b];
}

export function randomHexColor() {
    let [r, g, b] = randomRgbColor();

    let hr = r.toString(16).padStart(2, '0');
    let hg = g.toString(16).padStart(2, '0');
    let hb = b.toString(16).padStart(2, '0');

    return "#" + hr + hg + hb;
}