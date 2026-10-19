import { mount } from 'svelte'
import '@xyflow/svelte/dist/style.css'
import '$lib/styles/tokens.css'
import '$lib/styles/base.css'
import { listen } from '$lib/services/listen'
import App from './App.svelte'

listen() // the central event listener: every get:/post:/patch:/delete: event has its handler before the first read
mount(App, { target: document.getElementById('app')! })
