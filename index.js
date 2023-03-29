class PFInfoJS{
    constructor(name) {
        this.name = name;
    }
}

async function main() {
    let rust = await import("./pkg");
    let map = new Map();

    map.set("1233456", {"name": "i am object"});
    map.set("class version", new PFInfoJS("i am class"));

    console.log(rust.get_pf_map_from_js(map));
}
main()
