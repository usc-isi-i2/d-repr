import os
from logging.config import dictConfig

from flask import Flask, render_template
from peewee import DoesNotExist

from api.config import ROOT_DIR, HOME_DIR
from api.controllers.authentication import auth_bp
from api.controllers.dataset import dataset_bp
# from api.controllers.examples import examples_bp
from api.controllers.ontology import ont_bp
from api.controllers.resource import resource_bp
from api.misc.exception import BadRequest, UnauthorizedRequest, handle_does_not_exist_peewee, exception_handler, \
    not_implemented_handler

(HOME_DIR / "logs").mkdir(exist_ok=True)
dictConfig({
    'version': 1,
    'formatters': {
        'default': {
            'format': '[%(asctime)s] %(levelname)s in %(module)s: %(message)s',
        }
    },
    'handlers': {
        'console': {
            'level': 'DEBUG',
            'class': 'logging.StreamHandler',
            'formatter': 'default',
            'stream': 'ext://sys.stdout'
        },
        'file': {
            'level': 'DEBUG',
            'class': 'logging.handlers.RotatingFileHandler',
            'formatter': 'default',
            'filename': str(HOME_DIR / "logs" / "api.log"),
            'maxBytes': 1024,
            'backupCount': 3
        }
    },
    'loggers': {
        'default': {
            'level': 'DEBUG',
            'handlers': ['console', 'file']
        }
    },
    'root': {
        'level': 'DEBUG',
        'handlers': ['console', 'file']
    }
})

app = Flask("drepr-api",
            template_folder=str(ROOT_DIR / "api" / "flask_files" / "templates"),
            static_folder=str(ROOT_DIR / "api" / "flask_files" / "static"))
app.register_error_handler(BadRequest, BadRequest.flask_handler)
app.register_error_handler(UnauthorizedRequest, UnauthorizedRequest.flask_handler)
app.register_error_handler(DoesNotExist, handle_does_not_exist_peewee)
app.register_error_handler(NotImplementedError, not_implemented_handler)

if os.environ.get("FLASK_ENV", "") != "development":
    app.register_error_handler(Exception, exception_handler(app))

app.register_blueprint(auth_bp)
app.register_blueprint(dataset_bp)
app.register_blueprint(resource_bp)
app.register_blueprint(ont_bp)
# app.register_blueprint(examples_bp)


@app.route("/")
def index():
    return render_template("index.html")
