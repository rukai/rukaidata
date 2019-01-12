import * as three from 'three';
import OrbitControls from 'orbit-controls-es6';

const ANIMATION_FLAGS = {
    NONE:                      0x0,
    NO_OUT_TRANSITION:         0x1,
    LOOP:                      0x2,
    MOVES_CHARACTER:           0x4,
    FIXED_TRANSLATION:         0x8,
    FIXED_ROTATION:            0x10,
    FIXED_SCALE:               0x20,
    TRANSITION_OUT_FROM_START: 0x40,
    UNKNOWN:                   0x80
};

export class FighterRender {
    constructor(action_data) {
        const render_div = document.getElementById('fighter-render');

        this.scene = new three.Scene();
        this.camera = new three.PerspectiveCamera(40, 1, 1.0, 1000);
        this.controls = new OrbitControls(this.camera, render_div);
        this.controls.target.set(0, 8, 0);
        this.controls.update();
        this.face_right();

        this.renderer = new three.WebGLRenderer({ alpha: true });
        this.renderer.setClearColor(0xFFFFFF, 0);
        render_div.appendChild(this.renderer.domElement);

        this.window_resize();
        window.addEventListener('resize', this.window_resize, false);

        this.action_data = action_data;
        this.frame_index = 0;
        this.run = false;
        this.material = new three.MeshBasicMaterial({ color: 0xffff00, transparent: true, opacity: 0.5 });

        this.setup_frame();
        this.animate();
    }

    window_resize() {
        const render_div = document.getElementById('fighter-render');
        const width = render_div.offsetWidth;
        const height = width;

        this.camera.aspect = width / height;
        this.camera.updateProjectionMatrix();
        this.controls.update();
        this.renderer.setSize(width, height);
    }

    run_toggle() {
        if (this.run) {
            this.stop()
        }
        else {
            this.start()
        }
    }

    start() {
        const button = document.getElementById('run-toggle');
        button.innerHTML = "Stop";
        this.run = true;
    }

    stop() {
        const button = document.getElementById('run-toggle');
        button.innerHTML = "Run";
        this.run = false;
    }

    previous_frame() {
        this.stop();
        this.frame_index -= 1;
        if (this.frame_index == -1) {
            this.frame_index = this.action_data.frames.length - 1;
        }
        this.setup_frame();
    }

    next_frame() {
        this.stop();
        this.frame_index += 1;
        if (this.frame_index >= this.action_data.frames.length) {
            this.frame_index = 0;
        }
        this.setup_frame();
    }

    set_frame(index) {
        this.stop();
        this.frame_index = index;
        this.setup_frame();
    }

    face_left() {
        this.camera.position.set(40, 8, 0);
        this.controls.update();
    }

    face_right() {
        this.camera.position.set(-40, 8, 0);
        this.controls.update();
    }

    setup_frame() {
        const frame = this.action_data.frames[this.frame_index];

        // clear all objects from previous frame
        while (this.scene.children.length) {
            const child = this.scene.children[0];
            this.scene.remove(child);
            child.geometry.dispose();
        }

        // TODO: This will need to be heavily modified to display as rounded cubes.
        //       Render 8 sphere corners then connect them together by planes the length of the stretch value
        //       of that dimension.
        // generate hurtboxes
        for (let hurt_box of frame.hurt_boxes) {
            // The cuboid is generated with [0, 0, 0] at the center
            const diameter = hurt_box.hurt_box.radius * 2.0;
            const geometry = new three.BoxGeometry(diameter + Math.abs(hurt_box.hurt_box.stretch.x),
                                                   diameter + Math.abs(hurt_box.hurt_box.stretch.y),
                                                   diameter + Math.abs(hurt_box.hurt_box.stretch.z));
            const cube = new three.Mesh(geometry, this.material);

            const translation = new three.Matrix4();
            translation.makeTranslation(hurt_box.hurt_box.stretch.x + hurt_box.hurt_box.offset.x,
                                        hurt_box.hurt_box.stretch.y + hurt_box.hurt_box.offset.y,
                                        hurt_box.hurt_box.stretch.z + hurt_box.hurt_box.offset.z);

            const transform = new three.Matrix4();
            const bm = hurt_box.bone_matrix;
            transform.set(bm.x.x, bm.y.x, bm.z.x, bm.w.x,
                          bm.x.y, bm.y.y, bm.z.y, bm.w.y,
                          bm.x.z, bm.y.z, bm.z.z, bm.w.z,
                          bm.x.w, bm.y.w, bm.z.w, bm.w.w);
            transform.multiply(translation);

            const orientation = new three.Quaternion();
            orientation.setFromRotationMatrix(transform);

            cube.matrixAutoUpdate = false;
            cube.matrix.makeRotationFromQuaternion(orientation);
            cube.matrix.copyPosition(transform);
            this.scene.add(cube);
        }

        // update frame select
        for (var button of document.querySelectorAll('.frame-button')) {
            button.classList.remove('current-frame-button');
            if (parseInt(button.innerHTML, 10) == this.frame_index) {
                button.classList.add('current-frame-button');
            }
        }
    }

    animate() {
        if (this.run) {
            this.setup_frame();
            this.frame_index += 1;
            if (this.frame_index >= this.action_data.frames.length) {
                this.frame_index = 0;
            }
        }

        this.renderer.render(this.scene, this.camera);
        requestAnimationFrame(() => this.animate());
    }
}
