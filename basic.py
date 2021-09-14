import qti

qti_str = """<qti-choice-interaction max-choices="1" min-choices="1" response-identifier="RESPONSE">
  <qti-simple-choice identifier="A">Epinephrine</qti-simple-choice>
  <qti-simple-choice identifier="B">Glucagon</qti-simple-choice>
  <qti-simple-choice identifier="C">Insulin</qti-simple-choice>
  <qti-simple-choice identifier="D">Oxytocin</qti-simple-choice>
</qti-choice-interaction>"""

print(qti.render_qti_as_html3(qti_str))

