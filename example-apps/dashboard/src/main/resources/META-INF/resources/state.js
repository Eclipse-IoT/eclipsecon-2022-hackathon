class State {
    #target;
    #uri;
    #ws = null;
    #tid = null;

    constructor(target) {
        this.#target = target;
        const uri = new URL(document.documentURI);
        if (uri.protocol === "https:") {
            uri.protocol = "wss";
        } else {
            uri.protocol = "ws";
        }
        uri.hash = "";

        this.#uri = uri + "/api/updates/v1alpha1/events";

        console.info("Using WS URI:", this.#uri);

        this.#connect();
    }

    sortBy(column, current) {
        // the new direction
        let direction;
        if (current === "ascending") {
            direction = "descending";
        } else {
            direction = "ascending";
        }
        console.log("Sort by:", column);
        this.#ws?.send(JSON.stringify({request: "sortBy", column, direction}));
    }

    /**
     * Start connecting to the websocket
     */
    #connect() {
        const ws = new WebSocket(this.#uri);
        ws.onopen = () => {
            this.#setState({connected: true});
        };
        ws.onclose = (event) => {
            console.info("onClose", event);
            this.#setState({connected: false});
            this.#reconnect();
        }
        ws.onerror = (event) => {
            console.info("onError", event);
            this.#setState({connected: false});
            this.#reconnect();
        }
        ws.onmessage = (msg) => {
            this.#setState({connected: true, renderedState: msg.data});
        }
        this.#ws = ws;
    }

    #reconnect() {
        if (this.#tid === null) {
            this.#tid = window.setTimeout(() => {
                this.#tid = null;
                this.#connect();
            }, 5000);
        }
    }

    #setState(state) {
        this.state = state;
        if (this.state.renderedState !== undefined) {
            this.#target.innerHTML = this.state.renderedState;
        } else {
            this.#target.innerHTML = "";
        }

    }

    isConnected() {
        return this.state.connected;
    }
}

