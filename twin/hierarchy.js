if (context.newState.metadata.annotations === undefined) {
    context.newState.metadata.annotations = {};
}

if (context.newState.reportedState === undefined) {
    context.newState.reportedState = {};
}

function $ref() {
    return context.newState.metadata.name;
}

function normalize(group) {
    if (group === undefined) {
        return group;
    }
    return group.split('/').filter(t => t !== "")
}

function parentGroup(group) {
    if (group === undefined) {
        return group;
    }
    if (group.length > 0) {
        return group.slice(0,-1);
    } else {
        return undefined;
    }
}

function registerChild(reg, thing, $ref) {
    if (reg) {
        const deleting = context.newState.reconciliation.deleting["hierarchy"];
        const changed = context.newState.reconciliation.changed["hierarchy"];
        sendMessage(thing, {registerChild: {$ref, template: {
                    reconciliation: {
                        changed: {
                            hierarchy: changed,
                        },
                        deleting: {
                            hierarchy: deleting,
                        }
                    }
                }}});
    } else {
        sendMessage(thing, {unregisterChild: {$ref}});
    }
}

function registerChannel(reg, device, channel) {
    log(`Register channel: ${device} / ${channel} (${reg})`);

    if (reg) {
        if (context.newState.metadata.annotations["io.drogue/device"] !== device
            || context.newState.metadata.annotations["io.drogue/channel"] !== channel
        ) {
            context.newState.metadata.annotations["io.drogue/device"] = device;
            context.newState.metadata.annotations["io.drogue/channel"] = channel;
            registerChild(true, device, $ref())
        }

        context.newState.reportedState["$parent"] = {lastUpdate: new Date().toISOString(), value: device};
    } else {
        registerChild(false, device, $ref())
    }
}

function registerDevice(reg, device) {
    log(`Register device: ${device} (${reg})`);

    let group = normalize(context.newState.metadata.annotations["io.drogue/group"]);

    if (group !== undefined) {
        group = group.join('/');
        const parentStr = "/" + group;
        if (reg) {
            context.newState.metadata.annotations["io.drogue/device"] = device;
            if (context.currentState.metadata.annotations?.["io.drogue/group"] !== group) {
                registerChild(true, parentStr, $ref());
            }
            context.newState.reportedState["$parent"] = {lastUpdate: new Date().toISOString(), value: parentStr};
        } else {
            registerChild(false, parentStr, $ref());
        }
    }
}

function registerGroup(reg, group) {
    log(`Register group: ${group} (${reg})`);

    group = normalize(group);
    groupValue = group.join('/');
    const parent = parentGroup(group);

    if (parent !== undefined) {
        const parentStr = "/" + parent.join('/');
        if (reg) {
            if (context.newState.metadata.annotations["io.drogue/group"] !== groupValue) {
                context.newState.metadata.annotations["io.drogue/group"] = groupValue;

                registerChild(true, parentStr, $ref())
            }
            context.newState.reportedState["$parent"] = {lastUpdate: new Date().toISOString(), value: parentStr};
        } else {
            registerChild(false, parentStr, $ref())
        }
    }
}

function register(reg) {
    const ref = $ref();
    if (ref.startsWith('/')) {
        // group
        registerGroup(reg, ref);
    } else {
        const s = ref.split('/', 2);
        if (s.length >= 2) {
            // channel
            registerChannel(reg, s[0], s[1]);
        } else {
            // device
            registerDevice(reg, s[0]);
        }
    }
}

switch (context.action) {
    case "changed": {
        // FIXME: need to unregister previous state first, if it changed
        register(true);
        break;
    }
    case "deleting": {
        register(false);
        break;
    }
}