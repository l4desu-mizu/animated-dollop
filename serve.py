from pprint import pprint

import uvicorn
from fastapi import FastAPI
from starlette.requests import Request
from starlette.staticfiles import StaticFiles
from starlette.templating import Jinja2Templates

app = FastAPI()
app.mount("/static", StaticFiles(directory="./my_ui/dist/my_ui/browser"))

templates = Jinja2Templates(directory="./my_ui/dist/my_ui/browser")


@app.get("/")
def get(request: Request):
    return templates.TemplateResponse(
        request=request, name="index.html"
    )


if __name__ == '__main__':
    pprint(app.routes)
    uvicorn.run(app)
