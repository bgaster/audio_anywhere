const paramFilter = 0
const paramWave = 1
const paramRelation = 2
const paramSub = 3

function OnParamChange(param, value) {
    if (param == paramFilter) {
        client.renderer.updateFilter(value)
    }
    else if (param == paramWave) {
        client.renderer.updateWave(value)
    }
    else if (param == paramRelation) {
        client.renderer.updateRelation(value)
    }
    else if (param == paramSub) {
        client.renderer.updateSub(value)
    }
}

function OnControlChange(ctrlTag, value) {
    console.log("what the " + value)
}