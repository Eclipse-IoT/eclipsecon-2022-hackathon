const presence = context.newState.reportedState?.battery?.value?.flags?.presence;

if (presence !== undefined && presence !== "Unknown") {
    context.newState.reportedState?.battery?.value?.level
} else {
    null
}
