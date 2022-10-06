let lastUpdate = null;

let num = 0;
let debug = undefined; // can be set to []

for (const [key, value] of Object.entries(context.newState.reportedState || {})) {
    if (key.startsWith("$")) {
        continue;
    }

    try {
        num += 1;
        let timestamp = Date.parse(value.lastUpdate);

        if (debug) {
            debug.push({key, timestamp});
        }

        if ((lastUpdate === null) || (timestamp > lastUpdate)) {
            lastUpdate = timestamp;
        }
    } catch (error) {
        if (debug) {
            debug.push({key, error, lastUpdate: value.lastUpdate});
        }
    }
}

function result(name, diff) {
    const good = name === "GOOD";
    if (debug === undefined) {
        return {good, name};
    } else {
        return {good, name, diff, num, debug};
    }
}

if (lastUpdate === null) {

    result("BAD");

} else {

    const diff = Date.now() - lastUpdate;
    if (diff > 120000) {
        result("BAD", diff);
    } else if (diff > 20000) {
        result("STALE", diff);
    } else {
        result("GOOD", diff);
    }

}


