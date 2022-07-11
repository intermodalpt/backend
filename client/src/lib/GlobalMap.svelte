<script>
    import Box from './components/Box.svelte';
    import L from 'leaflet';
    import 'leaflet.markercluster';

    let map;
    let amlgeo;
    let parishesgeo;

    let mapLayers = {
        parishes: L.layerGroup(),
        municipalities: L.layerGroup(),
        stops: L.layerGroup()
    };

    let info = L.control();
    let zoom = 0;



    function zone_color(zone) {
        switch (zone) {
            case 1:
                return {color: "#f59f00"};
            case 2:
                return {color: "#0ca678"};
            case 3:
                return {color: "#ff6b00"};
            case 4:
                return {color: "#228be6"};
            default:
                return {color: "#6f7479"};
        }
    }


    function loadStops() {
        let bounds = map.getBounds();
        let x0 = bounds.getWest();
        let y0 = bounds.getNorth();
        let x1 = bounds.getEast();
        let y1 = bounds.getSouth();
        console.log(map.getBounds());
        fetch(`http://0.0.0.0:8080/api/stops/${x0}/${y0}/${x1}/${y1}`)
            .then(r => r.json())
            .then(nodes => {
                map.removeLayer(mapLayers.stops);
                mapLayers.stops = L.markerClusterGroup({
                    spiderfyOnMaxZoom: false,
                    showCoverageOnHover: false,
                    disableClusteringAtZoom: 14
                })
                nodes.forEach(node => {
                    mapLayers.stops.addLayer(L.marker([node.lat, node.lon]));
                });

                map.addLayer(mapLayers.stops);
            });
    }

    function onParishFeature(feature, layer) {
        layer.on({
            mouseover: (e) => {
                var layer = e.target;

                layer.setStyle({
                    weight: 5,
                    color: '#666',
                    dashArray: '',
                    fillOpacity: 0.7
                });

                layer.bringToFront();

                info.update(layer.feature.properties);
            },
            mouseout: (e) => {
                parishesgeo.resetStyle(e.target);
                info.update();
            },
            click: (e) => {
                let bounds = e.target.getBounds();
                map.fitBounds(bounds);
                loadStops();
            }
        });
    }

    function onMunicipalityFeature(feature, layer) {
        layer.on({
            mouseover: (e) => {
                let layer = e.target;

                layer.setStyle({
                    weight: 5,
                    color: '#666',
                    dashArray: '',
                    fillOpacity: 0.7
                });

                layer.bringToFront();

                info.update(layer.feature.properties);
            },
            mouseout: (e) => {
                amlgeo.resetStyle(e.target);
                info.update();
            },
            click: (e) => {
                map.fitBounds(e.target.getBounds());
                mapLayers.municipalities.removeFrom(map);
                mapLayers.parishes.addTo(map);
            }
        });
    }

    fetch('/src/assets/aml.geojson')
        .then(r => r.json())
        .then(obj => {
            amlgeo = L.geoJSON(obj, {
                style: (feature) => {
                    return zone_color(feature.properties.zone)
                },
                onEachFeature: onMunicipalityFeature
            }).addTo(mapLayers.municipalities);
            mapLayers.municipalities.addTo(map);
            map.fitBounds(amlgeo.getBounds());
        });

    fetch("/src/assets/freguesias.geojson")
        .then(x => x.json()).then(obj => {
        parishesgeo = L.geoJSON(obj, {
            // style: (feature) => {return zone_color(feature.properties.zone)},
            onEachFeature: onParishFeature
        }).addTo(mapLayers.parishes);
    });

    function createMap(container) {
        let m = L.map(container).setView([38.71856, -9.13720], 10);

        m.on('zoomstart', () => {
            let nextZoom = m.getZoom();
            alert(`From ${zoom} to ${nextZoom}`);
            zoom = nextZoom;
        })
        // m.on('zoomend', () => {
        //     let zoomLevel = map.getZoom();
        //     if (zoomLevel < 12) {
        //         map.add(mapLayers.municipalities);
        //     }
        // });
        L.tileLayer('https://tile.openstreetmap.bzh/br/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap'
        }).addTo(m);

        m.maxBounds = new L.LatLngBounds(new L.LatLng(38.3, -10.0), new L.LatLng(39.35, -8.0));
        m.maxBoundsViscosity = 1.0;
        m.minZoom = 10;
        m.setView([38.605, -9.0], 0);

        L.control.scale().addTo(m);


        info.onAdd = function (map) {
            this._div = L.DomUtil.create('div', 'info'); // create a div with a class "info"
            this.update();
            return this._div;
        };

        // method that we will use to update the control based on feature properties passed
        info.update = function (props) {
            if (props) {
                this._div.innerHTML = '<b>' + props.name + '</b><br />';
            }
        };
        info.addTo(m);


        var legend = L.control({position: 'bottomleft'});

        legend.onAdd = function (map) {
            const div = L.DomUtil.create('div', 'info legend');
            div.innerHTML = '' +
                '<i style="background:#f59f00"></i>Area 1<br>' +
                '<i style="background:#0ca678"></i>Area 2<br>' +
                '<i style="background:#ff6b00"></i>Area 3<br>' +
                '<i style="background:#228be6"></i>Area 4<br>' +
                '<i style="background:#abb3bb"></i>Independente';

            return div;
        };

        legend.addTo(m);

        return m;
    }

    function mapAction(container) {
        map = createMap(container);

        return {
            destroy: () => {
                map.remove();
                map = null;
            }
        };
    }
</script>


<link rel="stylesheet" href="https://unpkg.com/leaflet@1.8.0/dist/leaflet.css"
      integrity="sha512-xwE/Az9zrjBIphAcBb3F6JVqxf46+CDLwfLMHloNu6KEQCAWi6HcDUbeOfBIptF7tcCzusKFjFw2yuvEpDL9wQ=="
      crossorigin=""/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.css"/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.Default.css"/>
<link rel="stylesheet" href="/src/assets/css.css"/>
<h1>Mapa</h1>
<Box>
    <div class="map" use:mapAction></div>
</Box>



<style>
    .map {
        height: 650px;
        border-radius: 12px;
    }
</style>