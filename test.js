res = {
    send: function(){ },
    json: function(err){
        console.log("\n : " + err);
    },
    status: function(responseStatus) {
        assert.equal(responseStatus, 404);
        // This next line makes it chainable
        return this; 
    }
}