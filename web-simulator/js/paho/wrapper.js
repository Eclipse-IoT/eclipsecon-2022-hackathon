// A wrapper to make Paho JavaScript work in an ESM WASM environment
//
// We need this, as wasm-bindgen only imports from ESM, which is the cool new stuff in JavaScript-land.
// However, the JavaScript land is so fragmented, that there is "require()" and plain-old-browser stuff too.
// Some code (like Paho in this case) tries weird stuff, making tons of assumptions, so that it should work in
// the browser, using require, in NodeJS, ... however, all of that still isn't a proper module.
//
// Besides, constructors are not constructors but functions to function that create functions and stuff.
//
// So, what we do here is:
// * Assuming that the Paho library is loaded using a good-old-script tag
// * Creating a proper JavaScript module, which we can wasm-bindgen, making calls to the functions we expect to be present in the browser namespace

// I HATE JAVASCRIPT!

export class Client {
    constructor(endpoint, clientId) {
        try {
            this.client = new Paho.MQTT.Client(endpoint, clientId);
        }
        catch(e) {
            console.log(e);
            throw e;
        }
    }

    connect(opts) {
        // console.log("Connection options: ", opts);
        this.client.connect(opts);
    }

    disconnect() {
        this.client.disconnect();
    }

    get connected() {
        this.client.isConnected()
    }

    subscribe(filter, opts) {
        this.client.subscribe(filter, opts)
    }

    publish(topic, payload, qos, retained) {
        this.client.publish(topic, payload, qos, retained)
    }

    set onConnectionLost(handler) {
        this.client.onConnectionLost = handler;
    }

    set onMessageArrived(handler) {
        this.client.onMessageArrived = (msg) => {
            console.log(msg);
            handler(new Message(msg));
        };
    }
}

export class Message {
    constructor(msg) {
        this.msg = msg;
    }

    get topic () {
        return this.msg.topic;
    }

    get payloadBytes() {
        return this.msg.payloadBytes;
    }
}