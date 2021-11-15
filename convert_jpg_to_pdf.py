from PIL import Image


def convert_jpg_to_pdf(h_ID, title, pages):
    pdf=[]              #images converted for saving as pdf


    print("\r                                                                                                    ", end="")
    print(f"\rConverting {h_ID} to pdf...", end="", flush=True)
    for page_nr in range(1, pages+1):   #convert all saved images
        with Image.open(f"./{h_ID}/{h_ID}-{page_nr}.jpg") as img_file:              #open image
            pdf.append(img_file.convert("RGBA").convert("RGB"))                     #convert, append to pdf
    pdf[0].save(f"./{h_ID} {title}.pdf", save_all=True, append_images=pdf[1:])      #save
    print(f"\rConverted {h_ID} to pdf.", end="", flush=True)