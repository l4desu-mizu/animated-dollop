from pprint import pprint

import uvicorn
from fastapi import FastAPI
from starlette.staticfiles import StaticFiles


app = FastAPI()
app.mount("/", StaticFiles(directory="./my_ui/dist/my_ui/"))


if __name__ == '__main__':
    pprint(app.routes)
    uvicorn.run(app)
