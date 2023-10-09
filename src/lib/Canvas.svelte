<script>
	import * as L from "leaflet"
	import 'leaflet-draw'
	import 'leaflet-draw/dist/leaflet.draw.css'
	import 'leaflet/dist/leaflet.css';
	import {onMount} from "svelte";

	export let appContext
	onMount(() => {
		const img = new Image()
		img.src = appContext.archive
		let bounds = [[0,0], [img.height, img.width]];
		const map = L.map('map', {
			crs: L.CRS.Simple,
			maxZoom: 19,
			minZoom: -10,
			zoom: 1,
			zoomControl: false
		});
		L.imageOverlay(appContext.archive, bounds).addTo(map);
		map.fitBounds(bounds);
		map.addControl(L.control.zoom({ position: 'topright' }));
		let zones = new L.FeatureGroup();
		map.addLayer(zones);

		const options = {
			position: 'bottomright',
			draw: {
				polygon: {
					allowIntersection: true,
					drawError: {
						color: '#e1e100', // Color the shape will turn when intersects
						message: '<strong>Oh snap!<strong> you can\'t draw that!' // Message that will show when intersect
					},
					shapeOptions: {
						color: '#bada55'
					}
				},
				circle: false,
				rectangle: false,
				marker: false,
				polyline: false,
				circlemarker: false
			},
			edit: {
				featureGroup: zones, //REQUIRED!!
				remove: true
			}
		};
		var drawControl = new L.Control.Draw(options);
		map.addControl(drawControl);

		map.on(L.Draw.Event.CREATED, function (e) {
			zones.addLayer(e.layer);
			console.log(e.layer)
		});

	})
</script>

<!--<img src={appContext.archive} alt="">-->
<div id="map"></div>