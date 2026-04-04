class StructaResponse {
    statusCode = 200;
    body;
    headers = {};
    json(data) {
        this.body = data;
        this.setHeader('Content-Type', 'application/json');
        return this;
    }
    send(data) {
        this.body = data;
        return this;
    }
    status(code) {
        this.statusCode = code;
        return this;
    }
    setHeader(name, value) {
        this.headers[name] = value;
        return this;
    }
}
class StructaContext {
    request;
    response;
    next;
    params = {};
    query = {};
    constructor(request, response, next) {
        this.request = request;
        this.response = response;
        this.next = next;
    }
    get body() {
        return this.request.body;
    }
    status(code) {
        this.response.status(code);
        return this;
    }
    json(data) {
        this.response.json(data);
        return this;
    }
    send(data) {
        this.response.send(data);
        return this;
    }
}
export function createContext(request, next) {
    return new StructaContext(request, new StructaResponse(), next);
}
//# sourceMappingURL=context.js.map