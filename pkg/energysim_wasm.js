let wasm;

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error(`expected a boolean argument, found ${typeof(n)}`);
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

export function start() {
    wasm.start();
}

/**
 * @param {any} settings
 * @returns {SimulationResults}
 */
export function run_simulation(settings) {
    const ret = wasm.run_simulation(settings);
    return SimulationResults.__wrap(ret);
}

/**
 * @returns {string}
 */
export function get_sliders_json() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.get_sliders_json();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

const SimulationResultsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_simulationresults_free(ptr >>> 0, 1));

export class SimulationResults {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SimulationResults.prototype);
        obj.__wbg_ptr = ptr;
        SimulationResultsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SimulationResultsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_simulationresults_free(ptr, 0);
    }
    /**
     * @returns {Array<any>}
     */
    get years() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_years(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get pv_production() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_pv_production(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get pv_capacity() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_pv_capacity(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get wind_production() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_wind_production(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get wind_capacity() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_wind_capacity(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get nuclear_production() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_nuclear_production(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get nuclear_capacity() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_nuclear_capacity(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get other_lowc_production() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_other_lowc_production(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get consumo() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_consumo(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_years() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_years(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_pv_production() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_pv_production(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_wind_production() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_wind_production(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_hydro_production() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_hydro_production(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_geothermal_production() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_geothermal_production(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_pv_capacity() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_pv_capacity(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_wind_capacity() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_wind_capacity(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_hydro_capacity() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_hydro_capacity(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_geothermal_capacity() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_geothermal_capacity(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Array<any>}
     */
    get historical_bioenergy_capacity() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.simulationresults_historical_bioenergy_capacity(this.__wbg_ptr);
        return ret;
    }
}

const SimulationSettingsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_simulationsettings_free(ptr >>> 0, 1));

export class SimulationSettings {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SimulationSettingsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_simulationsettings_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get fotovoltaico_inizio() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_fotovoltaico_inizio(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set fotovoltaico_inizio(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_fotovoltaico_inizio(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get fotovoltaico_2030() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_fotovoltaico_2030(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set fotovoltaico_2030(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_fotovoltaico_2030(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get fotovoltaico_2035() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_fotovoltaico_2035(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set fotovoltaico_2035(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_fotovoltaico_2035(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get fotovoltaico_2040() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_fotovoltaico_2040(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set fotovoltaico_2040(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_fotovoltaico_2040(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get fotovoltaico_2050() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_fotovoltaico_2050(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set fotovoltaico_2050(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_fotovoltaico_2050(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get fotovoltaico_fine() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_fotovoltaico_fine(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set fotovoltaico_fine(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_fotovoltaico_fine(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get eolico_inizio() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_eolico_inizio(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set eolico_inizio(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_eolico_inizio(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get eolico_2030() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_eolico_2030(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set eolico_2030(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_eolico_2030(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get eolico_2035() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_eolico_2035(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set eolico_2035(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_eolico_2035(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get eolico_2040() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_eolico_2040(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set eolico_2040(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_eolico_2040(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get eolico_2050() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_eolico_2050(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set eolico_2050(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_eolico_2050(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get eolico_fine() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_eolico_fine(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set eolico_fine(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_eolico_fine(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get reattori_prima_learning() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_reattori_prima_learning(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set reattori_prima_learning(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(arg0);
        wasm.__wbg_set_simulationsettings_reattori_prima_learning(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get reattori_dopo_learning() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_reattori_dopo_learning(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set reattori_dopo_learning(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(arg0);
        wasm.__wbg_set_simulationsettings_reattori_dopo_learning(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get potenza_reattore() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_potenza_reattore(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set potenza_reattore(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_potenza_reattore(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get numero_reattori() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_numero_reattori(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set numero_reattori(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(arg0);
        wasm.__wbg_set_simulationsettings_numero_reattori(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get durata_cantiere_foak() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_durata_cantiere_foak(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set durata_cantiere_foak(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(arg0);
        wasm.__wbg_set_simulationsettings_durata_cantiere_foak(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get tempo_learning() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_tempo_learning(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set tempo_learning(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(arg0);
        wasm.__wbg_set_simulationsettings_tempo_learning(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get tasso_crescita_consumo() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_tasso_crescita_consumo(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set tasso_crescita_consumo(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_tasso_crescita_consumo(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get produzione_altre_fonti_lowc() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_produzione_altre_fonti_lowc(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set produzione_altre_fonti_lowc(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.__wbg_set_simulationsettings_produzione_altre_fonti_lowc(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get anno_inizio_installazioni_nucleare() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_anno_inizio_installazioni_nucleare(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set anno_inizio_installazioni_nucleare(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(arg0);
        wasm.__wbg_set_simulationsettings_anno_inizio_installazioni_nucleare(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get end_year() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.__wbg_get_simulationsettings_end_year(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set end_year(arg0) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(arg0);
        wasm.__wbg_set_simulationsettings_end_year(this.__wbg_ptr, arg0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_buffer_609cc3eee51ed158 = function() { return logError(function (arg0) {
        const ret = arg0.buffer;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_debug_e17b51583ca6a632 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.debug(arg0, arg1, arg2, arg3);
    }, arguments) };
    imports.wbg.__wbg_error_524f506f44df1645 = function() { return logError(function (arg0) {
        console.error(arg0);
    }, arguments) };
    imports.wbg.__wbg_error_80de38b3f7cc3c3c = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.error(arg0, arg1, arg2, arg3);
    }, arguments) };
    imports.wbg.__wbg_getwithrefkey_1dc361bd10053bfe = function() { return logError(function (arg0, arg1) {
        const ret = arg0[arg1];
        return ret;
    }, arguments) };
    imports.wbg.__wbg_info_033d8b8a0838f1d3 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.info(arg0, arg1, arg2, arg3);
    }, arguments) };
    imports.wbg.__wbg_instanceof_ArrayBuffer_e14585432e3737fc = function() { return logError(function (arg0) {
        let result;
        try {
            result = arg0 instanceof ArrayBuffer;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_instanceof_Uint8Array_17156bcf118086a9 = function() { return logError(function (arg0) {
        let result;
        try {
            result = arg0 instanceof Uint8Array;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_isSafeInteger_343e2beeeece1bb0 = function() { return logError(function (arg0) {
        const ret = Number.isSafeInteger(arg0);
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_length_a446193dc22c12f8 = function() { return logError(function (arg0) {
        const ret = arg0.length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_log_cad59bb680daec67 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.log(arg0, arg1, arg2, arg3);
    }, arguments) };
    imports.wbg.__wbg_new_78feb108b6472713 = function() { return logError(function () {
        const ret = new Array();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_a12002a7f91c75be = function() { return logError(function (arg0) {
        const ret = new Uint8Array(arg0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_push_737cfc8c1432c2c6 = function() { return logError(function (arg0, arg1) {
        const ret = arg0.push(arg1);
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_65595bdd868b3009 = function() { return logError(function (arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_warn_aaf1f4664a035bd6 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.warn(arg0, arg1, arg2, arg3);
    }, arguments) };
    imports.wbg.__wbindgen_as_number = function(arg0) {
        const ret = +arg0;
        return ret;
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = arg0;
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        _assertNum(ret);
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
        const ret = arg0 in arg1;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_2;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = arg0;
        const ret = typeof(val) === 'object' && val !== null;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = arg0 === undefined;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
        const ret = arg0 == arg1;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = arg1;
        const ret = typeof(obj) === 'number' ? obj : undefined;
        if (!isLikeNone(ret)) {
            _assertNum(ret);
        }
        getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = arg1;
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('energysim_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
