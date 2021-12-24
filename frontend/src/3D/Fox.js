import * as THREE from "three";
import { OBJLoader } from "../OBJLoader";
import { MTLLoader } from "../MTLLoader";
const manager = new THREE.LoadingManager();
const loader = new OBJLoader(manager)
const materialLoader = new MTLLoader(manager)
import { Heart } from "./Heart";
export class Fox {
    scene
    x = 0
    y = 0
    rotation = 0
    hearts = []
    floatAnimation = {
        isRunning: true,
        duration: msToDuration(1000),
        loops: [0, Number.MAX_SAFE_INTEGER],
        direction: 1,
        iteration: 0,
        speeds: {
            y: 0.005,
            x: 0,
            rotation: 0
        }
    }
    feedAnimation = {
        isRunning: false,
        duration: msToDuration(300),
        loops: [0, 2],
        direction: 1,
        iteration: 0,
        speeds: {
            y: 0.1,
            x: 0,
            rotation: 0
        }
    }
    object
    constructor(scene, x, y) {
        this.scene = scene
        this.x = x
        this.y = y
        this.load()
    }
    load = () => {
        materialLoader.load('Fox.mtl', (materials) => {
            materials.preload()
            loader.setMaterials(materials)
            loader.load('Fox.obj', (fox) => {
                this.scene.add(fox)
                this.object = fox
            })
        })
    }
    animate = () => {
        const { object, floatAnimation, feedAnimation } = this
        if (!object) return
        this.hearts.forEach(heart => {
            heart.animate()
        })
        if (floatAnimation.isRunning) this.floatAnimationTick()
    }
    feed = () => {
        this.hearts.forEach((heart) => {
            this.scene.remove(heart.object)
        })
        this.hearts  = new Array(3).fill(0).map(() => {
            const x = random(-3,3)
            const heart = new Heart(this.scene,x,random(-0.2,0.3),2)
            return heart
        })
        this.feedAnimation.isRunning = true
    }
    finishedAnimation = (anim) => {
        anim.direction = 1
        anim.loops[0] = 0
        anim.isRunning = false
        this.x = 0
        this.y = 0
        this.rotation = 0
        this.floatAnimation.isRunning = true
    }
    feedAnimationTick = () => {
        const { feedAnimation } = this
        this.animateObject(feedAnimation)
    }
    floatAnimationTick = () => {
        const { floatAnimation } = this
        this.animateObject(floatAnimation)
    }
    animateObject = (anim) => {
        const { object } = this
        Object.keys(anim.speeds).forEach(key => {
            const speed = anim.speeds[key]
            this[key] += speed * anim.direction

        })
        if (anim.duration < anim.iteration) {
            anim.iteration = 0
            anim.direction *= -1
            anim.loops[0]++
            if (anim.loops[0] >= anim.loops[1]) {
                this.finishedAnimation(anim)
            }
        }
        anim.iteration++
        object.position.y = this.y
        object.position.x = this.x
        object.rotateY(this.rotation)
    }
}

function random(min,max){
    return Math.random() * (max - min) + min
}

function msToDuration(ms) {
    return Math.floor(ms / (1000 / 60))
}