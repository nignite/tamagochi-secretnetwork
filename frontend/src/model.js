//@ts-nocheck
import * as THREE from "three"
import { Fox } from "./Fox"
import { OrbitControls } from './OrbitControls'
import { SecretAPI } from "./secret"
const dom = {
    btns: {
        left: document.querySelector("#btn-3"),
        center: document.querySelector("#btn-2"),
        right: document.querySelector("#btn-1"),
    },
    balance: document.querySelector('.fdt-amount'),
    saturation: document.querySelector('.saturation-level'),
    turbulence: document.querySelector("#turbulence"),
    status: document.querySelector('.status'),
    message: document.querySelector('.message')
}
const screen = document.getElementById("screen")
const scene = new THREE.Scene()
const camera = new THREE.PerspectiveCamera(60, 241 / 221, 0.1, 1000)
const renderer = new THREE.WebGLRenderer()
const node = new SecretAPI()
renderer.setPixelRatio(window.devicePixelRatio);


const controls = new OrbitControls(camera, renderer.domElement)
controls.autoRotate = true
controls.enableZoom = false

//Light
const hemiLight = new THREE.HemisphereLight(0xffffff, 0x444444, 1.4)
hemiLight.position.set(0, 300, 0)
scene.add(hemiLight)

const dirLight = new THREE.DirectionalLight(0xffffff, 1)
dirLight.position.set(40, 40, -20)
scene.add(dirLight)
renderer.setSize(241, 221)

renderer.setClearColor(0x412e71)

//background
screen.appendChild(renderer.domElement)
new THREE.CubeTextureLoader()
    .setPath('/cube/')
    .load(
        [
            '3x2y.png',
            '1x2y.png',
            '2x1y.png',
            '2x3y.png',
            '2x2y.png',
            '4x2y.png'
        ],
        function (cubeTexture) {
            // scene.background = cubeTexture
        }
    )

const fox = new Fox(scene, 0, 0)
camera.position.z = 6
camera.position.y = 1
const animate = function () {
    requestAnimationFrame(animate)
    fox.animate()
    renderer.render(scene, camera)

    animateBackground()
}

dom.btns.center.onclick = () => {
    fox.feed()
}
dom.btns.right.onclick = async () => {
    showMessage("Buying food...", 2000)
    await node.buyFood(10)
    update()
}
dom.btns.left.onclick = async () => {
    showMessage("Feeding pet...", 3000)
    try {
        await node.sendFood(10)
        update()
    } catch (err) {
        showMessage("It's not feeding time yet. ", 3000)
    }
}


const update = async () => {
    await updateBalance()
    await updateSaturationLevel()
}

const showMessage = (message, delay) => {
    dom.status.style.display = "block"
    dom.message.innerHTML = message
    setTimeout(() => {
        dom.message.innerHTML = ""
        dom.status.style.display = "none"
    }, delay)
}







const updateBalance = async () => {
    const new_balance = await node.getFoodBalance();
    dom.balance.innerHTML = new_balance
}
const updateSaturationLevel = async () => {
    const percentage = await node.getSaturationLevel()
    console.log(percentage)
    dom.saturation.innerHTML = `${percentage}%`
}

function animateBackground() {
    const baseFrequency = Number(dom.turbulence.getAttribute("baseFrequency"))
    dom.turbulence.setAttribute("baseFrequency", baseFrequency + 0.000005)
}


export { animate, update }