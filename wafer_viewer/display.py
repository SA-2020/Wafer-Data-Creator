from matplotlib import pyplot as plt
import numpy as np
import sys, os, tkinter, tkinter.filedialog, tkinter.messagebox



app_name = "Wafer Viewer"
root = tkinter.Tk()
root.withdraw()
fTyp = [("","*")]
filepath = os.path.abspath(os.path.dirname(__file__))
tkinter.messagebox.showinfo(app_name,'処理ファイルを選択してください！')
file = tkinter.filedialog.askopenfilename(filetypes = fTyp,initialdir = filepath)
tkinter.messagebox.showinfo(app_name, file)

a = np.loadtxt(file, delimiter=',',dtype={'names': ('x', 'y', 'ys'), 'formats': ('int', 'int', 'object')})

figure, axes = plt.subplots()

for i, j in enumerate(a):
    if j['ys'] == " true":
        j['ys']=1.0
    else:
        j['ys']=0.0

sample2 = a['ys'].astype(float).reshape(500,500)
axes.pcolor(sample2, cmap=plt.cm.Blues)
plt.savefig("test.png")
