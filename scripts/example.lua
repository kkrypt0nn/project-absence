ExampleModule = {}

function ExampleModule:noise_level()
    return "None"
end

function ExampleModule:description()
    return "This example module/script will just print 'Hello from Lua!' when a 'Ready' event has been emitted."
end

function ExampleModule:subscribers()
    return { "Ready" }
end

function ExampleModule:execute()
    println("$[fg:blue]$[effect:bold]Hello from Lua!$[reset]")
    return true
end

return ExampleModule
