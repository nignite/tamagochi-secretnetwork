import "./assets/styles/style.scss"
import { animate, update } from "./model"
import { SecretAPI } from './secret'
import { loadKelprOfflineSigner } from './keplr'

animate()
window.onload = () => {
    update()
    setInterval(update, 10000)
}




