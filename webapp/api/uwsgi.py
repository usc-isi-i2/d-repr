import os

from werkzeug.debug import DebuggedApplication
from api.app import app as application

if __name__ == '__main__':
    if os.environ.get('FLASK_ENV', '').lower() == 'development':
        print(">>> enable development server")
        application.wsgi_app = DebuggedApplication(application.wsgi_app, True)
    application.run()