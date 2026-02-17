ExampleScript = {}

function ExampleScript:description()
    return "This example module/script will add a 'hello_from' entry to the database entry of the discovered domain."
end

function ExampleScript:subscribers()
    return { "discovered:domain" }
end

function ExampleScript:execute(session, event)
    if event:is("discovered:domain") then
        local domain = event:domain()
        if not domain then
            logger:println("No domain found in event")
            return false
        end
        logger:println("New domain discovered: " .. domain)

        local db = session:database()
        db:add_data("domain", domain, "hello_from", "lua_script")
        logger:println("Added 'hello_from' to domain: " .. domain)
    end
    return true
end

return ExampleScript
