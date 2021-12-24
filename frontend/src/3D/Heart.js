import * as THREE from "three";
import { OBJLoader } from "../OBJLoader";
import { MTLLoader } from "../MTLLoader";
const manager = new THREE.LoadingManager();
const loader = new OBJLoader(manager)
const materialLoader = new MTLLoader(manager)
export class Heart {
    scene
    x = 0
    y = 0
    rotation = 0
    mainAnimation = {
        isRunning: true,
        duration: msToDuration(1800),
        loops: [0, 5],
        direction: 1,
        iteration: 0,
        speeds: {
            y: 0.03,
            x: 0,
            rotation: 0.001
        }
    }
    object
    constructor(scene, x, y,z) {
        this.scene = scene
        this.x = x
        this.y = y
        this.z = z
        this.load()
    }
    load = () => {
        return new Promise((resolve) => {
            loader.load('Heart.obj', (obj) => {
                this.scene.add(obj)
                this.object = obj
                obj.children.forEach((mesh) => {
                    mesh.material.color.setHex( 0xff0000 )
                    mesh.scale.set(0.3,0.3,0.3)

                })
                resolve(obj)
                obj.position.x = this.x
                obj.position.y = this.y
                obj.position.z = this.z
            })
        } )
    }
    animate = () => {
        const { object, mainAnimation } = this
        if (!object) return
        if (mainAnimation.isRunning) this.mainAnimationTick()
    }
    mainAnimationTick = () => {
        const { mainAnimation } = this
        this.animateObject(mainAnimation)
    }

    finishedAnimation = (anim) => {
        anim.direction = 1
        anim.loops[0] = 0
        this.y = 100
        this.object.position.y = this.y
        anim.isRunning = false
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

function msToDuration(ms) {
    return Math.floor(ms / (1000 / 60))
}