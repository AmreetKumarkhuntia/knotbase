import { mount } from 'svelte'
import '@xyflow/svelte/dist/style.css'
import './app.css'
import './theme.svelte' // sets data-theme before the first paint
import App from './App.svelte'

mount(App, { target: document.getElementById('app')! })
