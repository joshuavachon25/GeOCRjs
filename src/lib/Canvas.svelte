<script>
	import * as L from "leaflet"
	import 'leaflet-draw'
	import 'leaflet-draw/dist/leaflet.draw.css'
	import 'leaflet/dist/leaflet.css'
	import {onMount} from "svelte"

	export let appContext

	onMount(() => {
		const img = new Image()
		img.onload = function(){
			let bounds = [[0,0], [img.height, img.width]]
			const map = L.map('map', {
				crs: L.CRS.Simple,
				maxZoom: 19,
				minZoom: -10,
				zoom: 1,
				zoomControl: false,
				attributionControl: false
			})
			L.imageOverlay(appContext.archive, bounds).addTo(map)
			map.fitBounds(bounds)
			//map.addControl(L.control.zoom({ position: 'topright' }))
			let zones = new L.FeatureGroup()
			map.addLayer(zones)

			L.Control.CustomZoom = L.Control.extend({
				onAdd: function(map) {
					let wrap = L.DomUtil.create('div')
					wrap.classList.add('zoomWrap')
					let btn1 = L.DomUtil.create('button')
					btn1.innerText = "+"
					btn1.classList.add('btnZoomIn')
				 	wrap.appendChild(btn1)
					btn1.addEventListener('click', () => map.zoomIn(0.5))

					let btn2 = L.DomUtil.create('button')
					btn2.innerText = "-"
					btn2.classList.add('btnZoomOut')
					wrap.appendChild(btn2)
					btn2.addEventListener('click', () => map.zoomOut())

					return wrap;
				},

				onRemove: function(map) {
					// Nothing to do here
				}
			})

			L.control.customZoom = function(opts) {
				return new L.Control.CustomZoom(opts);
			}
			L.control.customZoom({ position: 'topright' }).addTo(map);
			const options = {
				position: 'bottomright',
				draw: {
					polygon: {
						allowIntersection: true,
						drawError: {
							color: '#e1e100',
							message: 'Erreur! Modifiez la forme de cette zone!'
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
					featureGroup: zones,
					remove: true
				}
			};
			var drawControl = new L.Control.Draw(options)
			map.addControl(drawControl)

			map.on(L.Draw.Event.CREATED, function (e) {
				zones.addLayer(e.layer)
				console.log(e.layer._latlngs[0])
				console.log(zones._layers)
			});
		}
		img.src = appContext.archive
	})

</script>


<div id="map"></div>