import { mount } from 'svelte'
import '@xyflow/svelte/dist/style.css'
import '$lib/styles/tokens.css'
import '$lib/styles/base.css'
import App from './App.svelte'

mount(App, { target: document.getElementById('app')! })
