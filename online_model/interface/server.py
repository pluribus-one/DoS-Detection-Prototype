from flask import Flask, render_template, request, jsonify
from flask_wtf import FlaskForm
from wtforms import SubmitField, IntegerField
from wtforms.validators import DataRequired, NumberRange
import random
import string
import requests

app = Flask(__name__)
app.config['SECRET_KEY'] = ''.join(random.choices(string.ascii_letters, k=20))

# --------------
# FORM
# --------------
class MetricsForm(FlaskForm):
    window1 = IntegerField('Window 1:', validators=[DataRequired(), NumberRange(min=1)])
    window2 = IntegerField('Window 2:', validators=[DataRequired(), NumberRange(min=1)])
    window3 = IntegerField('Window 3:', validators=[DataRequired(), NumberRange(min=1)])
    submit = SubmitField('Invia')

# ---------
# APIs
# ---------
@app.route('/', methods=['GET', 'POST'])
def index():
    form = MetricsForm()
    if request.method == 'POST' and form.validate_on_submit():
        window1 = form.window1.data
        window2 = form.window2.data
        window3 = form.window3.data

        data = {
            "count_first_window" : window1,
            "count_second_window": window2,
            "count_third_window" : window3,
        }

        try:
            response = requests.post(
                url = "http://firewall:8080/metrics",
                json=data
            )

            if response.status_code == 200:
                return "<h1>Success</h1>"
            else:
                return "<h1>Bad metrics</h1>"
        except:
            return "<h1>Firewall not found</h1>"

    return render_template('index.html', form=form)


if __name__ == '__main__':
    app.run("0.0.0.0", port=5000)

