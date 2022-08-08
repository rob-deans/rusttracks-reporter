import { Map, View } from 'ol';
import { Vector as VectorLayer } from 'ol/layer';
import Tile from 'ol/layer/Tile';
import { OSM, Vector } from 'ol/source';
import { transform, toLonLat, fromLonLat } from 'ol/proj';
import { Point, MultiLineString } from 'ol/geom';
import Feature from 'ol/Feature';
import { fromString } from 'ol/color';
import { Circle, Fill, Stroke, Style } from 'ol/style';
import Overlay from 'ol/Overlay';
import axios from 'axios';

let today = new Date();
let yesterday = new Date(new Date().setDate(today.getDate() - 1));

today = today.toISOString().slice(0, -8);
yesterday = yesterday.toISOString().slice(0, -8);
document.getElementById('start-date-input').value = yesterday;
document.getElementById('end-date-input').value = today;
let accValue = document.getElementById('acc-value');

let accSlider = document.getElementById("acc-slider");

var map = new Map({
    target: 'map',
    layers: [
        new Tile({
            source: new OSM()
        })
    ],
    view: new View({
        center: fromLonLat([-0.14, 51.5]),
        zoom: 10
    })
});


let vectorDotsLayer;
let vectorPathLayer;
var markers = [];
function getLocations() {
    map.removeLayer(vectorDotsLayer);
    map.removeLayer(vectorPathLayer);

    let start_date_input = document.getElementById('start-date-input').value;
    let end_date_input = document.getElementById('end-date-input').value;

    axios.get(process.env.API_URL + "/api/locations", { params: { start_date: start_date_input, end_date: end_date_input } })
        .then(response => {

            let data = response.data;
            for (let x in data) {
                let transformed_point = transform([data[x].lon, data[x].lat], 'EPSG:4326',
                    'EPSG:3857')
                let geom_point = new Point(transformed_point);
                markers.push(new Feature({
                    geometry: geom_point,
                    timestamp: data[x].tst,
                    accuracy: data[x].acc,
                }));

            };

            createDotLayer(markers);
            createPathLayer(markers);
            map.addLayer(vectorDotsLayer);
            bounceToInitialMarker(markers);

        })

};

function bounceToInitialMarker(markers) {
    if (markers.length === 0) {
        return;
    }
    let init_marker = markers[0].getGeometry().flatCoordinates
    const animationDuration = 750;
    map.getView().animate(
        { zoom: 10, duration: animationDuration },
        { center: init_marker, duration: animationDuration },
        { zoom: 15, duration: animationDuration }

    );
}

function calculateTimeDistance(markers) {
    if (markers.length > 1) {
        return markers[markers.length - 1].get('timestamp') - markers[0].get('timestamp');
    } else {
        return 0;
    }
}

function* calculateColour(startColour, endColour, step) {
    for (let i = 0; i < step; i++) {
        yield 'rgba(' +
            + Math.round(endColour[0] + (startColour[0] - endColour[0]) * (i / step)) + ','
            + Math.round(endColour[1] + (startColour[1] - endColour[1]) * (i / step)) + ','
            + Math.round(endColour[2] + (startColour[2] - endColour[2]) * (i / step)) + ','
            + (endColour[3] + (startColour[3] - endColour[3]) * (i / step))
            + ')';
    }
}

function drawStyle(feature) {
    var type = feature.getGeometry().getType();
    var lineStrings = [];
    var styles = [];
    if (type === "LineString") {
        lineStrings = [feature.getGeometry()];
    } else if (type === "MultiLineString") {
        lineStrings = feature.getGeometry().getLineStrings();
    }
    lineStrings.forEach(function (lineString) {
        var coordinates = lineString.getCoordinates();
        let colour_gen = calculateColour(fromString('blue'), fromString('red'), coordinates.length)
        for (var i = 0; i < coordinates.length - 1; i++) {
            let lineColour = colour_gen.next().value;
            styles.push(
                new ol.style.Style({
                    geometry: new geom.LineString(coordinates.slice(i, i + 2)),
                    stroke: new Stroke({
                        color: lineColour,
                    })
                })
            );
        }
    });

    return styles;
}

function displayOption(clickedBtn) {
    buttonToChange = clickedBtn == "dots" ? "path" : "dots"
    document.getElementById(buttonToChange).setAttribute("class", "display-option-btn");
    document.getElementById(clickedBtn).setAttribute("class", "display-option-btn-clicked");
    if (clickedBtn == "path") {
        map.removeLayer(vectorDotsLayer);
        map.addLayer(vectorPathLayer);
    } else if (clickedBtn == "dots") {
        map.removeLayer(vectorPathLayer);
        map.addLayer(vectorDotsLayer);
    } else if (clickedBtn == "pathanddots") {
        map.addLayer(vectorPathLayer);
        map.addLayer(vectorDotsLayer);
    }
}

function createDotLayer(markers) {
    var vectorSource = new Vector({
        features: markers
    });

    const fill = new Fill({
        color: 'rgba(0,213,255,0.4)',
    });
    var iconStyle = new Style({
        image: new Circle({
            fill: fill,
            radius: 5,

        }),
        fill: fill,
    });


    vectorDotsLayer = new VectorLayer({
        source: vectorSource,
        style: iconStyle
    });

}
function updateDotLayer(markers) {

    var vectorSource = new Vector({
        features: markers
    });
    vectorDotsLayer.setSource(vectorSource);
}

function geomPointsFromMarkers(markers) {
    let geom_points = [];
    for (let i = 0; i < markers.length; i++) {
        geom_points.push(markers[i].getGeometry().flatCoordinates);
    }
    return geom_points;

}
function createPathLayer(markers) {
    let geomPoints = geomPointsFromMarkers(markers);
    var geomLineStrings = new MultiLineString([geomPoints]);
    var featureGeom = new Feature({
        name: "Path",
        geometry: geomLineStrings,
    });
    var vectorPathSource = new Vector({});
    vectorPathSource.addFeature(featureGeom);
    vectorPathLayer = new VectorLayer({
        source: vectorPathSource,
        style: feature => drawStyle(feature)
    });
}

function updatePathLayer(markers) {
    let geomPoints = geomPointsFromMarkers(markers)
    var geomLineStrings = new MultiLineString([geomPoints]);
    var featureGeom = new Feature({
        name: "Path",
        geometry: geomLineStrings,
    });
    var vectorPathSource = new Vector({});
    vectorPathSource.addFeature(featureGeom);
    vectorPathLayer.setSource(vectorPathSource);
}

// Collapsible 
var coll = document.getElementsByClassName("collapsible");

for (let i = 0; i < coll.length; i++) {
    coll[i].addEventListener("click", function () {
        this.classList.toggle("active");
        document.getElementById("header").classList.toggle("collapsed");
        var content = this.nextElementSibling;
        if (content.style.maxHeight) {
            content.style.maxHeight = null;
        } else {
            content.style.maxHeight = content.scrollHeight + "px";
        }
    });
}


function createPopup(feature) {
    let coords = toLonLat(feature.getGeometry().flatCoordinates);
    let inner = `
                <div id="timestamp">
                    <p class="popup-subtitle"> TIMESTAMP </p>
                    <p class="popup-values"> ${new Date(feature.get('timestamp') * 1000).toUTCString()} </p>
                </div >
                <div id="coords">
                    <p class="popup-subtitle"> LOCATION </p>
                    <p class="popup-values"> (${coords[0].toFixed(6)}, ${coords[1].toFixed(6)}) </p>
                </div>
                <div id="accuracy">
                    <p class="popup-subtitle"> ACCURACY </p>
                    <p class="popup-values"> ${feature.get('accuracy')}m </p>
                </div>
            `
    popupContent.innerHTML = inner;
}


const container = document.getElementById('popup');
const popupContent = document.getElementById('popup-content');
var popup = new Overlay({
    element: container,
    positioning: 'bottom-center',
    stopEvent: false,
    offset: [0, -10],
});
map.addOverlay(popup);

map.on('pointermove', (event) => {
    var feature = map.forEachFeatureAtPixel(event.pixel,
        (feature, layer) => {
            return feature;
        }, { hitTolerance: 1 });
    if (feature && feature.get('name') != "Path") {
        let coords = feature.getGeometry().flatCoordinates;
        createPopup(feature);
        container.style.visibility = 'visible';
        popup.setPosition(coords);
    } else {
        popupContent.innerHTML = "";
        container.style.visibility = 'hidden';
    }
});

accSlider.addEventListener('input', function () {
    accValue.innerHTML = `${accSlider.value}m`;
});

accSlider.addEventListener('change', function () {
    if (markers.length > 1) {
        let filtered = markers.filter(marker => marker.get('accuracy') <= accSlider.value);
        updateDotLayer(filtered);
        updatePathLayer(filtered);
    }
});

let locationBtn = document.getElementById('get-location-btn');
locationBtn.addEventListener("click", function () {
    getLocations();
});

getLocations();
