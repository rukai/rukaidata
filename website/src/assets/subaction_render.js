"use strict";

class OrbitControls extends THREE.EventDispatcher {
  constructor (object, domElement) {
    super();

    this.object = object;
    this.domElement = (domElement !== undefined) ? domElement : document;

    // Set to false to disable this control
    this.enabled = true;

    // "target" sets the location of focus, where the object orbits around
    this.target = new THREE.Vector3();

    // How far you can dolly in and out ( PerspectiveCamera only )
    this.minDistance = 0;
    this.maxDistance = Infinity;

    // How far you can zoom in and out ( OrthographicCamera only )
    this.minZoom = 0;
    this.maxZoom = Infinity;

    // How far you can orbit vertically, upper and lower limits.
    // Range is 0 to Math.PI radians.
    this.minPolarAngle = 0; // radians
    this.maxPolarAngle = Math.PI; // radians

    // How far you can orbit horizontally, upper and lower limits.
    // If set, must be a sub-interval of the interval [ - Math.PI, Math.PI ].
    this.minAzimuthAngle = -Infinity; // radians
    this.maxAzimuthAngle = Infinity; // radians

    // Set to true to enable damping (inertia)
    // If damping is enabled, you must call controls.update() in your animation loop
    this.enableDamping = false;
    this.dampingFactor = 0.25;

    // This option actually enables dollying in and out; left as "zoom" for backwards compatibility.
    // Set to false to disable zooming
    this.enableZoom = true;
    this.zoomSpeed = 1.0;

    // Set to false to disable rotating
    this.enableRotate = true;
    this.rotateSpeed = 1.0;

    // Set to false to disable panning
    this.enablePan = true;
    this.keyPanSpeed = 7.0; // pixels moved per arrow key push

    // Set to true to automatically rotate around the target
    // If auto-rotate is enabled, you must call controls.update() in your animation loop
    this.autoRotate = false;
    this.autoRotateSpeed = 2.0; // 30 seconds per round when fps is 60

    // Set to false to disable use of the keys
    this.enableKeys = true;

    // The four arrow keys
    this.keys = {
      LEFT: 37,
      UP: 38,
      RIGHT: 39,
      BOTTOM: 40
    };

    // Mouse buttons
    this.mouseButtons = {
      ORBIT: THREE.MOUSE.LEFT,
      ZOOM: THREE.MOUSE.MIDDLE,
      PAN: THREE.MOUSE.RIGHT
    };

    // for reset
    this.target0 = this.target.clone();
    this.position0 = this.object.position.clone();
    this.zoom0 = this.object.zoom;

    //
    // public methods
    //

    this.getPolarAngle = () => spherical.phi;
    this.getAzimuthalAngle = () => spherical.theta;

    this.reset = function () {
      scope.target.copy(scope.target0);
      scope.object.position.copy(scope.position0);
      scope.object.zoom = scope.zoom0;

      scope.object.updateProjectionMatrix();
      scope.dispatchEvent(changeEvent);

      scope.update();

      state = STATE.NONE;
    };

    // this method is exposed, but perhaps it would be better if we can make it private...
    this.update = (function () {
      var offset = new THREE.Vector3();

      // so camera.up is the orbit axis
      var quat = new THREE.Quaternion().setFromUnitVectors(object.up, new THREE.Vector3(0, 1, 0));
      var quatInverse = quat.clone().inverse();

      var lastPosition = new THREE.Vector3();
      var lastQuaternion = new THREE.Quaternion();

      return function update () {
        var position = scope.object.position;

        offset.copy(position).sub(scope.target);

        // rotate offset to "y-axis-is-up" space
        offset.applyQuaternion(quat);

        // angle from z-axis around y-axis
        spherical.setFromVector3(offset);

        if (scope.autoRotate && state === STATE.NONE) {
          rotateLeft(getAutoRotationAngle());
        }

        spherical.theta += sphericalDelta.theta;
        spherical.phi += sphericalDelta.phi;

        // restrict theta to be between desired limits
        spherical.theta = Math.max(scope.minAzimuthAngle, Math.min(scope.maxAzimuthAngle, spherical.theta));

        // restrict phi to be between desired limits
        spherical.phi = Math.max(scope.minPolarAngle, Math.min(scope.maxPolarAngle, spherical.phi));
        spherical.makeSafe();
        spherical.radius *= scale;

        // restrict radius to be between desired limits
        spherical.radius = Math.max(scope.minDistance, Math.min(scope.maxDistance, spherical.radius));

        // move target to panned location
        scope.target.add(panOffset);

        offset.setFromSpherical(spherical);

        // rotate offset back to "camera-up-vector-is-up" space
        offset.applyQuaternion(quatInverse);

        position.copy(scope.target).add(offset);

        scope.object.lookAt(scope.target);

        if (scope.enableDamping === true) {
          sphericalDelta.theta *= (1 - scope.dampingFactor);
          sphericalDelta.phi *= (1 - scope.dampingFactor);
        } else {
          sphericalDelta.set(0, 0, 0);
        }

        scale = 1;
        panOffset.set(0, 0, 0);

        // update condition is:
        // min(camera displacement, camera rotation in radians)^2 > EPS
        // using small-angle approximation cos(x/2) = 1 - x^2 / 8

        if (zoomChanged ||
          lastPosition.distanceToSquared(scope.object.position) > EPS ||
          8 * (1 - lastQuaternion.dot(scope.object.quaternion)) > EPS) {
          scope.dispatchEvent(changeEvent);

          lastPosition.copy(scope.object.position);
          lastQuaternion.copy(scope.object.quaternion);
          zoomChanged = false;

          return true
        }

        return false
      }
    }());

    this.dispose = function () {
      scope.domElement.removeEventListener('contextmenu', onContextMenu, false);
      scope.domElement.removeEventListener('mousedown', onMouseDown, false);
      scope.domElement.removeEventListener('wheel', onMouseWheel, false);

      scope.domElement.removeEventListener('touchstart', onTouchStart, false);
      scope.domElement.removeEventListener('touchend', onTouchEnd, false);
      scope.domElement.removeEventListener('touchmove', onTouchMove, false);

      document.removeEventListener('mousemove', onMouseMove, false);
      document.removeEventListener('mouseup', onMouseUp, false);

      window.removeEventListener('keydown', onKeyDown, false);
    };

    //
    // internals
    //

    var scope = this;
    var changeEvent = {
      type: 'change'
    };
    var startEvent = {
      type: 'start'
    };
    var endEvent = {
      type: 'end'
    };

    var STATE = {
      NONE: -1,
      ROTATE: 0,
      DOLLY: 1,
      PAN: 2,
      TOUCH_ROTATE: 3,
      TOUCH_DOLLY: 4,
      TOUCH_PAN: 5
    };

    var state = STATE.NONE;

    var EPS = 0.000001;

    // current position in spherical coordinates
    var spherical = new THREE.Spherical();
    var sphericalDelta = new THREE.Spherical();

    var scale = 1;
    var panOffset = new THREE.Vector3();
    var zoomChanged = false;

    var rotateStart = new THREE.Vector2();
    var rotateEnd = new THREE.Vector2();
    var rotateDelta = new THREE.Vector2();

    var panStart = new THREE.Vector2();
    var panEnd = new THREE.Vector2();
    var panDelta = new THREE.Vector2();

    var dollyStart = new THREE.Vector2();
    var dollyEnd = new THREE.Vector2();
    var dollyDelta = new THREE.Vector2();

    function getAutoRotationAngle () {
      return 2 * Math.PI / 60 / 60 * scope.autoRotateSpeed
    }

    function getZoomScale () {
      return Math.pow(0.95, scope.zoomSpeed)
    }

    function rotateLeft (angle) {
      sphericalDelta.theta -= angle;
    }

    function rotateUp (angle) {
      sphericalDelta.phi -= angle;
    }

    var panLeft = (function () {
      var v = new THREE.Vector3();

      return function panLeft (distance, objectMatrix) {
        v.setFromMatrixColumn(objectMatrix, 0); // get X column of objectMatrix
        v.multiplyScalar(-distance);

        panOffset.add(v);
      }
    }());

    var panUp = (function () {
      var v = new THREE.Vector3();

      return function panUp (distance, objectMatrix) {
        v.setFromMatrixColumn(objectMatrix, 1); // get Y column of objectMatrix
        v.multiplyScalar(distance);

        panOffset.add(v);
      }
    }());

    // deltaX and deltaY are in pixels; right and down are positive
    var pan = (function () {
      var offset = new THREE.Vector3();

      return function pan (deltaX, deltaY) {
        var element = scope.domElement === document ? scope.domElement.body : scope.domElement;

        if (Object.getPrototypeOf(scope.object).isPerspectiveCamera) {
          // perspective
          var position = scope.object.position;
          offset.copy(position).sub(scope.target);
          var targetDistance = offset.length();

          // half of the fov is center to top of screen
          targetDistance *= Math.tan((scope.object.fov / 2) * Math.PI / 180.0);

          // we actually don't use screenWidth, since perspective camera is fixed to screen height
          panLeft(2 * deltaX * targetDistance / element.clientHeight, scope.object.matrix);
          panUp(2 * deltaY * targetDistance / element.clientHeight, scope.object.matrix);
        } else if (Object.getPrototypeOf(scope.object).isOrthographicCamera) {
          // orthographic
          panLeft(deltaX * (scope.object.right - scope.object.left) / scope.object.zoom / element.clientWidth, scope.object.matrix);
          panUp(deltaY * (scope.object.top - scope.object.bottom) / scope.object.zoom / element.clientHeight, scope.object.matrix);
        } else {
          // camera neither orthographic nor perspective
          console.warn('WARNING: OrbitControls.js encountered an unknown camera type - pan disabled.');
          scope.enablePan = false;
        }
      }
    }());

    function dollyIn (dollyScale) {
      if (Object.getPrototypeOf(scope.object).isPerspectiveCamera) {
        scale /= dollyScale;
      } else if (Object.getPrototypeOf(scope.object).isOrthographicCamera) {
        scope.object.zoom = Math.max(scope.minZoom, Math.min(scope.maxZoom, scope.object.zoom * dollyScale));
        scope.object.updateProjectionMatrix();
        zoomChanged = true;
      } else {
        console.warn('WARNING: OrbitControls.js encountered an unknown camera type - dolly/zoom disabled.');
        scope.enableZoom = false;
      }
    }

    function dollyOut (dollyScale) {
      if (Object.getPrototypeOf(scope.object).isPerspectiveCamera) {
        scale *= dollyScale;
      } else if (Object.getPrototypeOf(scope.object).isOrthographicCamera) {
        scope.object.zoom = Math.max(scope.minZoom, Math.min(scope.maxZoom, scope.object.zoom / dollyScale));
        scope.object.updateProjectionMatrix();
        zoomChanged = true;
      } else {
        console.warn('WARNING: OrbitControls.js encountered an unknown camera type - dolly/zoom disabled.');
        scope.enableZoom = false;
      }
    }

    //
    // event callbacks - update the object state
    //

    function handleMouseDownRotate (event) {
      rotateStart.set(event.clientX, event.clientY);
    }

    function handleMouseDownDolly (event) {
      dollyStart.set(event.clientX, event.clientY);
    }

    function handleMouseDownPan (event) {
      panStart.set(event.clientX, event.clientY);
    }

    function handleMouseMoveRotate (event) {
      rotateEnd.set(event.clientX, event.clientY);
      rotateDelta.subVectors(rotateEnd, rotateStart);
      var element = scope.domElement === document ? scope.domElement.body : scope.domElement;

      // rotating across whole screen goes 360 degrees around
      rotateLeft(2 * Math.PI * rotateDelta.x / element.clientWidth * scope.rotateSpeed);

      // rotating up and down along whole screen attempts to go 360, but limited to 180
      rotateUp(2 * Math.PI * rotateDelta.y / element.clientHeight * scope.rotateSpeed);
      rotateStart.copy(rotateEnd);
      scope.update();
    }

    function handleMouseMoveDolly (event) {
      dollyEnd.set(event.clientX, event.clientY);
      dollyDelta.subVectors(dollyEnd, dollyStart);

      if (dollyDelta.y > 0) {
        dollyIn(getZoomScale());
      } else if (dollyDelta.y < 0) {
        dollyOut(getZoomScale());
      }

      dollyStart.copy(dollyEnd);
      scope.update();
    }

    function handleMouseMovePan (event) {
      panEnd.set(event.clientX, event.clientY);
      panDelta.subVectors(panEnd, panStart);
      pan(panDelta.x, panDelta.y);
      panStart.copy(panEnd);
      scope.update();
    }

    function handleMouseWheel (event) {
      if (event.deltaY < 0) {
        dollyOut(getZoomScale());
      } else if (event.deltaY > 0) {
        dollyIn(getZoomScale());
      }

      scope.update();
    }

    function handleKeyDown (event) {
      switch (event.keyCode) {
        case scope.keys.UP:
          pan(0, scope.keyPanSpeed);
          scope.update();
          break

        case scope.keys.BOTTOM:
          pan(0, -scope.keyPanSpeed);
          scope.update();
          break

        case scope.keys.LEFT:
          pan(scope.keyPanSpeed, 0);
          scope.update();
          break

        case scope.keys.RIGHT:
          pan(-scope.keyPanSpeed, 0);
          scope.update();
          break
      }
    }

    function handleTouchStartRotate (event) {
      rotateStart.set(event.touches[0].pageX, event.touches[0].pageY);
    }

    function handleTouchStartDolly (event) {
      var dx = event.touches[0].pageX - event.touches[1].pageX;
      var dy = event.touches[0].pageY - event.touches[1].pageY;

      var distance = Math.sqrt(dx * dx + dy * dy);

      dollyStart.set(0, distance);
    }

    function handleTouchStartPan (event) {
      panStart.set(event.touches[0].pageX, event.touches[0].pageY);
    }

    function handleTouchMoveRotate (event) {
      rotateEnd.set(event.touches[0].pageX, event.touches[0].pageY);
      rotateDelta.subVectors(rotateEnd, rotateStart);

      var element = scope.domElement === document ? scope.domElement.body : scope.domElement;

      // rotating across whole screen goes 360 degrees around
      rotateLeft(2 * Math.PI * rotateDelta.x / element.clientWidth * scope.rotateSpeed);

      // rotating up and down along whole screen attempts to go 360, but limited to 180
      rotateUp(2 * Math.PI * rotateDelta.y / element.clientHeight * scope.rotateSpeed);

      rotateStart.copy(rotateEnd);

      scope.update();
    }

    function handleTouchMoveDolly (event) {
      var dx = event.touches[0].pageX - event.touches[1].pageX;
      var dy = event.touches[0].pageY - event.touches[1].pageY;

      var distance = Math.sqrt(dx * dx + dy * dy);

      dollyEnd.set(0, distance);

      dollyDelta.subVectors(dollyEnd, dollyStart);

      if (dollyDelta.y > 0) {
        dollyOut(getZoomScale());
      } else if (dollyDelta.y < 0) {
        dollyIn(getZoomScale());
      }

      dollyStart.copy(dollyEnd);

      scope.update();
    }

    function handleTouchMovePan (event) {
      panEnd.set(event.touches[0].pageX, event.touches[0].pageY);
      panDelta.subVectors(panEnd, panStart);
      pan(panDelta.x, panDelta.y);
      panStart.copy(panEnd);
      scope.update();
    }

    //
    // event handlers - FSM: listen for events and reset state
    //

    function onMouseDown (event) {
      if (scope.enabled === false) return

      event.preventDefault();

      if (event.button === scope.mouseButtons.ORBIT) {
        if (scope.enableRotate === false) return

        handleMouseDownRotate(event);

        state = STATE.ROTATE;
      } else if (event.button === scope.mouseButtons.ZOOM) {
        if (scope.enableZoom === false) return

        handleMouseDownDolly(event);

        state = STATE.DOLLY;
      } else if (event.button === scope.mouseButtons.PAN) {
        if (scope.enablePan === false) return

        handleMouseDownPan(event);

        state = STATE.PAN;
      }

      if (state !== STATE.NONE) {
        document.addEventListener('mousemove', onMouseMove, false);
        document.addEventListener('mouseup', onMouseUp, false);

        scope.dispatchEvent(startEvent);
      }
    }

    function onMouseMove (event) {
      if (scope.enabled === false) return

      event.preventDefault();

      if (state === STATE.ROTATE) {
        if (scope.enableRotate === false) return

        handleMouseMoveRotate(event);
      } else if (state === STATE.DOLLY) {
        if (scope.enableZoom === false) return

        handleMouseMoveDolly(event);
      } else if (state === STATE.PAN) {
        if (scope.enablePan === false) return

        handleMouseMovePan(event);
      }
    }

    function onMouseUp (event) {
      if (scope.enabled === false) return

      document.removeEventListener('mousemove', onMouseMove, false);
      document.removeEventListener('mouseup', onMouseUp, false);

      scope.dispatchEvent(endEvent);

      state = STATE.NONE;
    }

    function onMouseWheel (event) {
      if (scope.enabled === false || scope.enableZoom === false || (state !== STATE.NONE && state !== STATE.ROTATE)) return

      event.preventDefault();
      event.stopPropagation();
      handleMouseWheel(event);

      scope.dispatchEvent(startEvent); // not sure why these are here...
      scope.dispatchEvent(endEvent);
    }

    function onKeyDown (event) {
      if (scope.enabled === false || scope.enableKeys === false || scope.enablePan === false) return
      handleKeyDown(event);
    }

    function onTouchStart (event) {
      if (scope.enabled === false) return

      switch (event.touches.length) {
        case 1: // one-fingered touch: rotate
          if (scope.enableRotate === false) return
          handleTouchStartRotate(event);
          state = STATE.TOUCH_ROTATE;

          break

        case 2: // two-fingered touch: dolly

          if (scope.enableZoom === false) return
          handleTouchStartDolly(event);
          state = STATE.TOUCH_DOLLY;

          break

        case 3: // three-fingered touch: pan

          if (scope.enablePan === false) return
          handleTouchStartPan(event);
          state = STATE.TOUCH_PAN;
          break

        default:

          state = STATE.NONE;
      }

      if (state !== STATE.NONE) {
        scope.dispatchEvent(startEvent);
      }
    }

    function onTouchMove (event) {
      if (scope.enabled === false) return

      event.preventDefault();
      event.stopPropagation();

      switch (event.touches.length) {
        case 1: // one-fingered touch: rotate

          if (scope.enableRotate === false) return
          if (state !== STATE.TOUCH_ROTATE) return // is this needed?...
          handleTouchMoveRotate(event);
          break

        case 2: // two-fingered touch: dolly

          if (scope.enableZoom === false) return
          if (state !== STATE.TOUCH_DOLLY) return // is this needed?...
          handleTouchMoveDolly(event);
          break

        case 3: // three-fingered touch: pan

          if (scope.enablePan === false) return
          if (state !== STATE.TOUCH_PAN) return // is this needed?...
          handleTouchMovePan(event);
          break

        default:
          state = STATE.NONE;
      }
    }

    function onTouchEnd (event) {
      if (scope.enabled === false) return
      scope.dispatchEvent(endEvent);
      state = STATE.NONE;
    }

    function onContextMenu (event) {
      event.preventDefault();
    }

    scope.domElement.addEventListener('contextmenu', onContextMenu, false);
    scope.domElement.addEventListener('mousedown', onMouseDown, false);
    scope.domElement.addEventListener('wheel', onMouseWheel, false);
    scope.domElement.addEventListener('touchstart', onTouchStart, false);
    scope.domElement.addEventListener('touchend', onTouchEnd, false);
    scope.domElement.addEventListener('touchmove', onTouchMove, false);

    window.addEventListener('keydown', onKeyDown, false);

    // force an update at start
    this.update();
  };
}

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

const hitbox_color0 = 0xEF6400; // orange
const hitbox_color1 = 0xFF0000; // red
const hitbox_color2 = 0xFF00FF; // purple
const hitbox_color3 = 0x18d6c9; // turqoise
const hitbox_color4 = 0x24d618; // green

class FighterRender {
    constructor(subaction_data, subaction_extent) {
        const render_div = document.getElementById('fighter-render');

        const extent_up = parseFloat(this.get_from_url("up"));
        if (!Number.isNaN(extent_up)) {
            subaction_extent.up = extent_up;
        }
        this.set_in_url("up", subaction_extent.up);

        const extent_down = parseFloat(this.get_from_url("down"));
        if (!Number.isNaN(extent_down)) {
            subaction_extent.down = extent_down;
        }
        this.set_in_url("down", subaction_extent.down);

        const extent_left = parseFloat(this.get_from_url("left"));
        if (!Number.isNaN(extent_left)) {
            subaction_extent.left = extent_left;
        }
        this.set_in_url("left", subaction_extent.left);

        const extent_right = parseFloat(this.get_from_url("right"));
        if (!Number.isNaN(extent_right)) {
            subaction_extent.right = extent_right;
        }
        this.set_in_url("right", subaction_extent.right);

        this.subaction_data = subaction_data;
        this.subaction_extent = subaction_extent;

        this.extent_middle_y = (subaction_extent.up   + subaction_extent.down) / 2;
        this.extent_middle_z = (subaction_extent.left + subaction_extent.right) / 2;
        this.extent_height = this.subaction_extent.up    - this.subaction_extent.down;
        this.extent_width  = this.subaction_extent.right - this.subaction_extent.left;
        this.extent_aspect = this.extent_width / this.extent_height;
        this.fov = 40.0;

        this.scene = new THREE.Scene();
        this.scene_overlay = new THREE.Scene();
        this.camera = new THREE.PerspectiveCamera(this.fov, 1, 1, 1000); // The values here dont really matter, the camera gets overwritten in the later call to this.window_resize()
        this.controls = new OrbitControls(this.camera, render_div);
        this.controls.update();

        this.renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
        this.renderer.setClearColor(0xFFFFFF, 0);
        this.renderer.autoClear = false;
        render_div.appendChild(this.renderer.domElement);

        window.addEventListener('resize', () => this.window_resize(), false);

        this.frame_index = parseInt(this.get_from_url("frame"), 10);
        // handle invalid frame index
        if (Number.isNaN(this.frame_index) || this.frame_index < 0 || this.frame_index >= this.subaction_data.frames.length) {
            this.frame_index = 0;
        }

        this.invulnerable_select = document.getElementById('invulnerable-select');
        var invuln = this.get_from_url("invuln");
        if (invuln == null) {
            invuln = "Hit"
        }
        this.invulnerable_select.value = invuln;

        this.ecb_checkbox = document.getElementById('ecb-checkbox');
        this.ecb_checkbox.checked = this.get_bool_from_url("ecb");

        this.wireframe_checkbox = document.getElementById('wireframe-checkbox');
        this.wireframe_checkbox.checked = this.get_bool_from_url("wireframe");

        this.perspective_checkbox = document.getElementById('perspective-checkbox');
        this.perspective_checkbox.checked = this.get_bool_from_url("perspective");

        this.run = false;
        this.prev_timestamp = 0;
        this.ecb_material        = new THREE.MeshBasicMaterial({ color: 0xf15c0a, transparent: false, side: THREE.DoubleSide });
        this.transn_material     = new THREE.MeshBasicMaterial({ color: 0xffffff, transparent: false, side: THREE.DoubleSide });
        this.transn_max_material = new THREE.MeshBasicMaterial({ color: 0x00ff00, transparent: false, side: THREE.DoubleSide }); // Used when the lower ecb point is capped by the transN y position
        this.ledge_grab_box_material = new THREE.MeshBasicMaterial({ color: 0xffffff, transparent: true, opacity: 0.5, side: THREE.DoubleSide });

        // Manually call these callbacks to initialize stuff
        this.window_resize();
        this.face_right();
        this.wireframe_toggle();
        this.perspective_toggle();

        this.setup_frame();
        requestAnimationFrame((timestamp) => this.animate(timestamp));
    }

    window_resize() {
        const render_div = document.getElementById('fighter-render');
        const width = render_div.offsetWidth;
        let height = width;
        if (height > 750) {
            height = 750;
        }
        this.aspect = width / height;

        var radius = Math.max(
            this.subaction_extent.up    - this.extent_middle_y,
            this.subaction_extent.right - this.extent_middle_z
        );
        const fov_rad = this.fov * Math.PI / 180.0;
        // The new value will be used on next call to face_left() or face_right()
        this.camera_distance = radius / Math.tan(fov_rad / 2.0);

        // This logic probably only works because width >= height is always true
        if (this.extent_aspect > this.aspect) {
            this.camera_distance /= this.aspect;
        }
        else if (this.extent_width > this.extent_height) {
            this.camera_distance /= this.extent_aspect;
        }

        this.recreate_camera();
        this.controls.update();
        this.renderer.setSize(width, height);
    }

    recreate_camera() {
        const old_position = this.camera.position;
        if (this.perspective_checkbox.checked) {
            this.camera = new THREE.PerspectiveCamera(this.fov, this.aspect, 1, 1000);
            this.camera.position.copy(old_position);
            this.controls.object = this.camera;
            this.controls.update();
        }
        else {
            var height = this.extent_height;
            var width  = this.extent_width;

            if (this.extent_aspect > this.aspect) {
                // keep width at max size
                // shrink height to keep aspect ratio
                height = width / this.aspect;
            }
            else {
                // keep height at max size
                // shrink width to keep aspect ratio
                width = height * this.aspect;
            }
            this.camera = new THREE.OrthographicCamera(
                -width/2.0,
                width/2.0,
                height/2.0,
                -height/2.0,
                1,
                1000
            );
            this.camera.position.copy(old_position);
            this.controls.object = this.camera;
            this.controls.update();
        }
    }

    set_invulnerable_type(value) {
        this.setup_frame();
        this.set_in_url("invuln", this.invulnerable_select.value);
    }

    wireframe_toggle() {
        if (this.wireframe_checkbox.checked) {
            this.hurtbox_normal_material     = new THREE.MeshBasicMaterial({ color: 0xffff00,      wireframe: true });
            this.hurtbox_intangible_material = new THREE.MeshBasicMaterial({ color: 0x0000ff,      wireframe: true });
            this.hurtbox_invincible_material = new THREE.MeshBasicMaterial({ color: 0x00ff00,      wireframe: true });
            this.hitbox_material0            = new THREE.MeshBasicMaterial({ color: hitbox_color0, wireframe: true });
            this.hitbox_material1            = new THREE.MeshBasicMaterial({ color: hitbox_color1, wireframe: true });
            this.hitbox_material2            = new THREE.MeshBasicMaterial({ color: hitbox_color2, wireframe: true });
            this.hitbox_material3            = new THREE.MeshBasicMaterial({ color: hitbox_color3, wireframe: true });
            this.hitbox_material4            = new THREE.MeshBasicMaterial({ color: hitbox_color4, wireframe: true });
        }
        else {
            this.hurtbox_normal_material     = new THREE.MeshBasicMaterial({ color: 0xffff00,      transparent: true, opacity: 0.4 });
            this.hurtbox_intangible_material = new THREE.MeshBasicMaterial({ color: 0x0000ff,      transparent: true, opacity: 0.4 });
            this.hurtbox_invincible_material = new THREE.MeshBasicMaterial({ color: 0x00ff00,      transparent: true, opacity: 0.4 });
            this.hitbox_material0            = new THREE.MeshBasicMaterial({ color: hitbox_color0, transparent: true, opacity: 0.5 });
            this.hitbox_material1            = new THREE.MeshBasicMaterial({ color: hitbox_color1, transparent: true, opacity: 0.5 });
            this.hitbox_material2            = new THREE.MeshBasicMaterial({ color: hitbox_color2, transparent: true, opacity: 0.5 });
            this.hitbox_material3            = new THREE.MeshBasicMaterial({ color: hitbox_color3, transparent: true, opacity: 0.5 });
            this.hitbox_material4            = new THREE.MeshBasicMaterial({ color: hitbox_color4, transparent: true, opacity: 0.5 });
        }
        this.setup_frame();
        this.set_bool_in_url("wireframe", this.wireframe_checkbox.checked);
    }

    ecb_toggle() {
        this.setup_frame();
        this.set_bool_in_url("ecb", this.ecb_checkbox.checked);
    }

    perspective_toggle() {
        this.recreate_camera();
        this.setup_frame();
        this.set_bool_in_url("perspective", this.perspective_checkbox.checked);
    }

    run_toggle() {
        if (this.run) {
            // this.animate() increments this.frame_index after calling this.setup_frame() to avoid skipping the first frame.
            // However that means frame_index is ahead of where it should be.
            // So before this.stop() we decrement this.frame_index back to where it was.
            this.frame_index -= 1;
            if (this.frame_index == -1) {
                this.frame_index = this.subaction_data.frames.length - 1;
            }

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
        this.run_time = 0.0;
    }

    stop() {
        const button = document.getElementById('run-toggle');
        button.innerHTML = "Run";

        this.run = false;

        this.set_in_url("frame", Math.max(0, Math.min(this.subaction_data.frames.length-1, this.frame_index)));
    }

    previous_frame() {
        this.frame_index -= 1;
        if (this.frame_index == -1) {
            this.frame_index = this.subaction_data.frames.length - 1;
        }
        this.stop();
        this.setup_frame();
    }

    next_frame() {
        this.frame_index += 1;
        if (this.frame_index >= this.subaction_data.frames.length) {
            this.frame_index = 0;
        }
        this.stop();
        this.setup_frame();
    }

    set_frame(index) {
        this.frame_index = index - 1;
        this.stop();
        this.setup_frame();
    }

    face_left() {
        this.controls.reset();
        this.controls.target.set(0,                    this.extent_middle_y, this.extent_middle_z);
        this.camera.position.set(this.camera_distance, this.extent_middle_y, this.extent_middle_z);
        this.controls.update();
    }

    face_right() {
        this.controls.reset();
        this.controls.target.set(0,                     this.extent_middle_y, this.extent_middle_z);
        this.camera.position.set(-this.camera_distance, this.extent_middle_y, this.extent_middle_z);
        this.controls.update();
    }

    setup_frame() {
        const frame = this.subaction_data.frames[this.frame_index];

        // clear all objects from previous frame
        while (this.scene.children.length) {
            const child = this.scene.children[0];
            this.scene.remove(child);
            child.geometry.dispose();
        }
        while (this.scene_overlay.children.length) {
            const child = this.scene_overlay.children[0];
            this.scene_overlay.remove(child);
            child.geometry.dispose();
        }

        // generate ecb
        if (this.ecb_checkbox.checked) {
            const mid_y = (frame.ecb.top + frame.ecb.bottom) / 2.0;

            // referenced bones
            {
                const vertices = [
                    0, frame.ecb.top,    0,
                    0, mid_y,            frame.ecb.left,
                    0, mid_y,            frame.ecb.right,
                    0, frame.ecb.bottom, 0,
                ];

                const indices = [
                    0, 1, 2,
                    1, 2, 3,
                ];

                const geometry = new THREE.BufferGeometry();
                geometry.addAttribute('position', new THREE.BufferAttribute(new Float32Array(vertices), 3));
                geometry.setIndex(indices);

                const mesh = new THREE.Mesh(geometry, this.ecb_material);
                mesh.position.z = frame.x_pos;
                mesh.position.y = frame.y_pos;
                mesh.renderOrder = 6;
                this.scene_overlay.add(mesh);
            }

            // transN
            {
                const geometry = new THREE.CircleGeometry(0.3, 20);

                var material = this.transn_material;
                if (frame.ecb.transn_y == frame.ecb.bottom) {
                    material = this.transn_max_material;
                }
                var mesh = new THREE.Mesh(geometry, material);
                mesh.position.z = frame.x_pos + frame.ecb.transn_x;
                mesh.position.y = frame.y_pos + frame.ecb.transn_y;
                mesh.rotateY(Math.PI/2);
                mesh.renderOrder = 5;
                this.scene_overlay.add(mesh);
            }
        }

        // generate hitboxes
        for (let hit_box of frame.hit_boxes) {
            // hit/grab box specific logic
            var material = this.hitbox_material0;
            if (hit_box.hitbox_id == 1) {
                material = this.hitbox_material1;
            }
            if (hit_box.hitbox_id == 2) {
                material = this.hitbox_material2;
            }
            if (hit_box.hitbox_id == 3) {
                material = this.hitbox_material3;
            }
            if (hit_box.hitbox_id == 4) {
                material = this.hitbox_material4;
            }

            // only display hitboxes that are used in regular matches
            const hit_values = hit_box.next_values.Hit;
            if (hit_values != null && !hit_values.enabled) {
                continue;
            }

            var prev = null;
            const next = new THREE.Vector3(hit_box.next_pos.x, hit_box.next_pos.y + frame.y_pos, hit_box.next_pos.z + frame.x_pos);
            if (hit_box.prev_pos != null) {
                prev = new THREE.Vector3(hit_box.prev_pos.x, hit_box.prev_pos.y + frame.y_pos, hit_box.prev_pos.z + frame.x_pos);
            }
            const transform = new THREE.Matrix4();

            this.draw_cylinder(prev, next, hit_box.next_size, transform, material);
        }

        // generate hurtboxes
        const transform_translation_frame = new THREE.Matrix4();
        transform_translation_frame.makeTranslation(
            0.0,
            frame.y_pos,
            frame.x_pos,
        );
        for (let hurt_box of frame.hurt_boxes) {
            const bm = hurt_box.bone_matrix;
            const bone_matrix = new THREE.Matrix4();
            bone_matrix.set(
                bm.x.x, bm.y.x, bm.z.x, bm.w.x,
                bm.x.y, bm.y.y, bm.z.y, bm.w.y,
                bm.x.z, bm.y.z, bm.z.z, bm.w.z,
                bm.x.w, bm.y.w, bm.z.w, bm.w.w
            );

            var material = this.hurtbox_normal_material;
            if (hurt_box.state == "IntangibleFlashing" || hurt_box.state == "IntangibleNoFlashing" || hurt_box.state == "IntangibleQuickFlashing") {
                material = this.hurtbox_intangible_material;
            }
            else if (hurt_box.state == "Invincible") {
                material = this.hurtbox_invincible_material;
            }
            else if (this.invulnerable_select.value == "Grab" && !hurt_box.hurt_box.grabbable) {
                material = this.hurtbox_intangible_material;
            }
            else if (this.invulnerable_select.value == "Trap Item" && !hurt_box.hurt_box.trap_item_hittable) {
                material = this.hurtbox_intangible_material;
            }

            // Ah ... so its less of an offset + stretch and more like two seperate independent offsets.
            const prev_raw = hurt_box.hurt_box.offset;
            const next_raw = hurt_box.hurt_box.stretch;
            const radius = hurt_box.hurt_box.radius;

            const prev = new THREE.Vector3(prev_raw.x, prev_raw.y, prev_raw.z);
            const next = new THREE.Vector3(next_raw.x, next_raw.y, next_raw.z);

            const transform = new THREE.Matrix4();
            transform.copy(transform_translation_frame);
            transform.multiply(bone_matrix);

            this.draw_cylinder(prev, next, radius, transform, material);
        }

        // ledge grab box
        {
            const box = frame.ledge_grab_box;
            if (box != null) {
                const vertices = [
                    0, box.up,   box.left,
                    0, box.up,   box.right,
                    0, box.down, box.left,
                    0, box.down, box.right,
                ];

                const indices = [
                    0, 1, 2,
                    1, 2, 3,
                ];

                const geometry = new THREE.BufferGeometry();
                geometry.addAttribute('position', new THREE.BufferAttribute(new Float32Array(vertices), 3));
                geometry.setIndex(indices);

                const mesh = new THREE.Mesh(geometry, this.ledge_grab_box_material);
                mesh.position.z = frame.x_pos;
                mesh.position.y = frame.y_pos;
                mesh.renderOrder = 7;
                this.scene_overlay.add(mesh);
            }
        }


        // update frame select
        for (var button of document.querySelectorAll('.frame-button')) {
            button.classList.remove('current-frame-button');
            if (parseInt(button.innerHTML, 10) - 1 == this.frame_index) {
                button.classList.add('current-frame-button');
            }
        }
    }

    draw_cylinder(prev, next, radius, external_transform, material) {
        const vertices = [];
        const indices = [];
        var widthSegments = 23;
        var heightSegments = 17;

        // Make the wireframes less busy in wireframe mode
        if (this.wireframe_checkbox.checked) {
            widthSegments = 11;
            heightSegments = 7;
        }

        var prev_distance = 0;
        if (prev != null) {
            prev_distance = next.distanceTo(prev);
        }

        const grid = []
        var index_offset = 0;
        // modified UV sphere generation from:
        // https://github.com/mrdoob/THREE.js/blob/4ca3860851d0cd33535afe801a1aa856da277f3a/src/geometries/SphereGeometry.js
        for (var iy = 0; iy <= heightSegments; iy++) {
            var verticesRow = [];
            var v = iy / heightSegments;

            for (var ix = 0; ix <= widthSegments; ix++) {
                var u = ix / widthSegments;
                var y_offset = 0;
                if (prev != null) {
                    if (v >= 0 && v <= 0.5) {
                        y_offset += prev_distance;
                    }
                }

                const sin_v_pi = Math.sin(v * Math.PI);
                vertices.push(radius * Math.cos(u * Math.PI * 2) * sin_v_pi);
                vertices.push(radius * Math.cos(v * Math.PI) + y_offset);
                vertices.push(radius * Math.sin(u * Math.PI * 2) * sin_v_pi);

                verticesRow.push(index_offset++);
            }
            grid.push(verticesRow);
        }

        for (var iy = 0; iy < heightSegments; iy++) {
            for (var ix = 0; ix < widthSegments; ix++) {
                var a = grid[iy][ix + 1];
                var b = grid[iy][ix];
                var c = grid[iy + 1][ix];
                var d = grid[iy + 1][ix + 1];

                indices.push(a, b, d);
                indices.push(b, c, d);
            }
        }

        const geometry = new THREE.BufferGeometry();
        geometry.addAttribute('position', new THREE.BufferAttribute(new Float32Array(vertices), 3));
        geometry.setIndex(indices);

        const mesh = new THREE.Mesh(geometry, material);

        const rotation = new THREE.Quaternion();
        if (prev != null) {
            const diff = prev.clone();
            diff.sub(next);
            diff.normalize();
            rotation.setFromUnitVectors(new THREE.Vector3(0, 1, 0), diff);
        }

        const internal_transform = new THREE.Matrix4();
        internal_transform.compose(next, rotation, new THREE.Vector3(1, 1, 1));

        const transform = new THREE.Matrix4();
        transform.copy(external_transform);
        transform.multiply(internal_transform);

        mesh.matrixAutoUpdate = false;
        mesh.matrix.copy(transform);

        this.scene.add(mesh);
    }

    animate(timestamp) {
        if (this.previous_timestamp == 0) {
            this.previous_timestamp = timestamp;
            requestAnimationFrame((timestamp) => this.animate(timestamp));
            return;
        }

        if (this.run) {
            this.run_time += timestamp - this.previous_timestamp;
            if (this.run_time > 1000 / 60) {
                // this.frame_index needs to be incremented after this.setup_frame() to avoid skipping the first frame
                this.setup_frame();
                this.frame_index += 1;
                this.run_time -= 1000 / 60;
            }

            // loop animation
            if (this.frame_index >= this.subaction_data.frames.length) {
                this.frame_index = 0;
            }
        }

        this.renderer.clear();
        this.renderer.render(this.scene, this.camera);
        this.renderer.clearDepth();
        this.renderer.render(this.scene_overlay, this.camera);

        this.previous_timestamp = timestamp;
        requestAnimationFrame((timestamp) => this.animate(timestamp));
    }

    set_in_url(name, data) {
        var url = new URL(location);
        url.searchParams.set(name, data);
        window.history.replaceState({}, "", url);
    }

    set_bool_in_url(name, data) {
        var url = new URL(location);
        if (data) {
            url.searchParams.set(name, data);
        } else {
            url.searchParams.delete(name);
        }
        window.history.replaceState({}, "", url);
    }

    get_from_url(name) {
        var url = new URL(location);
        return url.searchParams.get(name);
    }

    get_bool_from_url(name) {
        var url = new URL(location);
        return url.searchParams.get(name) === "true";
    }
}

window.fighter_render = new FighterRender(fighter_subaction_data, fighter_subaction_extent);

const arrow_radius = 20;
function draw_angle(ctx, x, y, angle_degrees, color) {
    const angle_radians = angle_degrees / 180 * Math.PI;
    ctx.strokeStyle = color;
    ctx.fillStyle = color;
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.arc(x, y, 2, 0, Math.PI * 2);
    ctx.fill();

    ctx.beginPath();
    ctx.moveTo(x, y);
    const head_x = x + Math.cos(angle_radians) * arrow_radius;
    const head_y = y - Math.sin(angle_radians) * arrow_radius;
    ctx.lineTo(head_x, head_y);
    ctx.moveTo(head_x + Math.cos(angle_radians + Math.PI + 0.4) * arrow_radius / 2, head_y - Math.sin(angle_radians + Math.PI + 0.4) * arrow_radius / 2);
    ctx.lineTo(head_x, head_y);
    ctx.lineTo(head_x + Math.cos(angle_radians + Math.PI - 0.4) * arrow_radius / 2, head_y - Math.sin(angle_radians + Math.PI - 0.4) * arrow_radius / 2);
    ctx.stroke();
}

for (var hitbox_angle_render of document.getElementsByClassName('hitbox-angle-render')) {
    const angle_degrees = hitbox_angle_render.getAttribute("angle");
    const hitbox_id     = hitbox_angle_render.getAttribute("hitbox-id");
    var color = "#FFFFFF"
    if (hitbox_id == "0") {
        color = "#EF6400";
    }
    else if (hitbox_id == "1") {
        color = "#FF0000";
    }
    else if (hitbox_id == "2") {
        color = "#FF00FF";
    }
    else if (hitbox_id == "3") {
        color = "#18d6c9";
    }
    else if (hitbox_id == "4") {
        color = "#24d618";
    }
    const ctx = hitbox_angle_render.getContext("2d");

    if (angle_degrees == 361) {
        hitbox_angle_render.width = arrow_radius * 4;
        hitbox_angle_render.height = arrow_radius * 2;
        draw_angle(ctx, arrow_radius, arrow_radius, 0, color);
        draw_angle(ctx, arrow_radius*3, arrow_radius, 44, color);
    }
    else {
        hitbox_angle_render.width = arrow_radius * 2;
        hitbox_angle_render.height = arrow_radius * 2;
        draw_angle(ctx, arrow_radius, arrow_radius, angle_degrees, color);
    }
}
