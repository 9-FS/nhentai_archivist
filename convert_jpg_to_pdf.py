from PIL import Image, ImageFile, UnidentifiedImageError
import os


def convert_jpg_to_pdf(h_ID, title, pages):
    conversion_success=True #conversion successful? default yes
    pdf=[]                  #images converted for saving as pdf


    ImageFile.LOAD_TRUNCATED_IMAGES=True    #true setzen sonst wirft er beim Laden mancher Bilder eine unnötige Exception

    for page_nr in range(1, pages+1):   #convert all saved images
        print("\r                                                                                                    ", end="")
        print(f"\rConverting {h_ID}-{page_nr}.jpg to pdf...", end="", flush=True)
        try:
            with Image.open(f"./{h_ID}/{h_ID}-{page_nr}.jpg") as img_file:  #open image
                pdf.append(img_file.convert("RGBA").convert("RGB"))         #convert, append to pdf
        except UnidentifiedImageError:                  #download failed earlier, image is corrupted
            print(f"\nConverting {h_ID}-{page_nr}.jpg to pdf failed.")
            print(f"Removing corrupted image {h_ID}-{page_nr}.jpg to redownload later...", end="", flush=True)
            os.remove(f"./{h_ID}/{h_ID}-{page_nr}.jpg") #remove image, redownload later
            print("\r                                                                                                    ", end="")
            print(f"\rRemoved corrupted image {h_ID}-{page_nr}.jpg to redownload later.")
            conversion_success=False    #converting unsuccessful
        except FileNotFoundError:
            print(f"\n{h_ID}-{page_nr}.jpg not found, converting to pdf failed. Redownloading later.")
            conversion_success=False    #converting unsuccessful
    
    if conversion_success==False:   #if converting unsuccessful: don't create corrupt PDF
        return False
    
    print("\r                                                                                                    ", end="")
    print(f"\rSaving {h_ID}.pdf...", end="", flush=True)
    try:
        pdf[0].save(f"./{h_ID} {title}.pdf", save_all=True, append_images=pdf[1:])  #save
    except IndexError:
        return False
    return True         #conversion success