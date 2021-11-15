# nhentai downloader
# Date modified: January 18, 2020

from lxml import html
from PIL import Image
from sys import platform
import requests
import os
import shutil
import sys
import datetime


class DownloadHandler:
    def __init__(self, id_num):
        self.id_num = id_num
        page = requests.get(f'https://nhentai.net/g/{self.id_num}/')
        tree = html.fromstring(page.content)
        try:
            # if the page doesn't exist, the following will throw an error
            title = str(tree.xpath('//div[@id="info"]/h1/span[@class="pretty"]/text()')[0])
            try:
                title += str(tree.xpath('//div[@id="info"]/h1/span[@class="after"]/text()')[0])
            except:
                pass
            self._title = title
            # print(len(tree.xpath('//div[@class="thumb-container"]')))
            self._pages = int(
                len(tree.xpath('//div[@class="thumb-container"]')))
            self._valid = True
            # self._valid = False # DEBUG ONLY
        except:
            self._valid = False
        return

    def save_image(self, at_page, destination):
        curr_page = f"https://nhentai.net/g/{self.id_num}/{at_page}/"
        page = requests.get(curr_page)
        tree = html.fromstring(page.content)
        img_link = tree.xpath(
            '//section[@id="image-container"]/a/img/@src')
        # Save image to temp folder
        img_file = os.path.join(destination, f"{at_page}.png")
        temp_img = open(img_file, 'wb')
        temp_img.write(requests.get(img_link[0]).content)
        temp_img.close()
        return img_file

    @property
    def title(self):
        return self._title

    @property
    def valid(self):
        return self._valid

    @property
    def pages(self):
        return self._pages


class PDFHandler:
    def save_to_pdf(self, images, output_path):
        converted = []
        for img_num, img in enumerate(images):
            # print(f'Converting {img_num}')  Debug only
            converted.append(img.convert('RGBA').convert('RGB'))
        first_page = converted[0]
        converted.remove(first_page)
        first_page.save(output_path, save_all=True,
                        append_images=converted)
        return


class PathHandler:
    def __init__(self, folder_path: str, temp_path: str, name: str, id_num: int):
        self.path_dir = folder_path
        self.bad_chars = ['*', ':', '?', '.', '"', '|', '/', '\\', '<', '>']
        self.file_name = self.__problem_char_rm(name)
        # print(self.__set_path())
        self._final_path = self.__set_path()
        self._temp_path = os.path.join(temp_path, f'temp-{id_num}')

    @property
    def valid(self):
        return len(self._final_path) < 200

    @property
    def unique(self):
        return not os.path.exists(self._final_path)

    @property
    def temp_path(self):
        return self._temp_path

    @property
    def final_path(self):
        return self._final_path

    def rename_path(self, name):
        self.file_name = self.__problem_char_rm(name)
        self._final_path = self.__set_path()

    def __set_path(self):
        return os.path.join(f'{self.path_dir}', f'{self.file_name}.pdf')

    def __problem_char_rm(self, address: str) -> str:
        """
        Function to remove problematic characters of a path.
        Any characters that causes the "windows can't create this path because it
        contains illegal characters" should be removed here
        Parameters
        ----------
        address : str
            The path string
        Returns
        -------
        str
            The address with the characters removed
        """
        result = address
        for char in self.bad_chars:
            # go through each character in the set and replace with nothing
            result = result.replace(char, "")
        return result


def open_folder(folder_path: str):
    if platform == "darwin":
        os.system(f'open {folder_path}/')
    elif platform == "win32":
        os.system(f'start {folder_path}\\')
    return


def show_help():
    message = f"""
            nhentai downloader pdf - help
            [ Prompt Usage ]
            [ To Download ]
            - Enter in the ID number(s) of the doujin you wish to download.
            Note: Doujins musct come from nhentai website.
                  This script will not work with any other site.
            Hint: ID numbers can be found in the URL.
                  https://nhentai.net/g/[id number]/
            - To create a queue (multiple downloads) enter all the ID numbers
            Note: The order of the ID's does not matter.
              on the same input field separated by a space
            - Hit enter to begin downloading
            Note: If a doujin has the same name as an already downloaded item,
                  then it will skip that download.
                  If a doujin has a name that is too long, a warning will
                  appear and prompt to enter a new name.
            Usage:
            Enter number(s): [ID number(s) ... ]
            Example:
            Enter number(s): 111111 222222 333333
            [ Other Options ]
            When prompted, you may enter one of the other commands:
            - done : this will end execution of the program
            - help : this will display this text
            - open : this will open finder/files/file explorer to the
                     default download folder
            """
    print(message)
    return


def process_queue(dl_queue, output_folder, temp_folder, log):
    for currPos, id_num in enumerate(dl_queue, 1):
        # Create log statement
        date = datetime.datetime.now()
        log_statement = f'{date} | ID: {id_num} '

        # Get doujin info
        print(f'[ Fetching {id_num} ({currPos} / {len(dl_queue)}) ]')
        dl_handler = DownloadHandler(id_num)
        if not dl_handler.valid:
            print('ERROR - Doujin not found. Skipped\n')
            log_statement += '[ERROR] Doujin not found.\n'
            log.write(log_statement)
            continue
        print(f'Title: {dl_handler.title}')
        print(f'Pages: {dl_handler.pages}')

        # Check to see if file exist
        path_handler = PathHandler(
            output_folder, temp_folder, dl_handler.title, id_num)
        if not path_handler.unique:
            print("ERROR - File already exist. Skipped.\n")
            log_statement += '[ERROR] File already exist.\n'
            log.write(log_statement)
            continue
        # Check to see if path is too long
        while not path_handler.valid or not path_handler.unique:
            while not path_handler.valid:
                title = input(
                    "⚠️   WARNING - File path is too long! Please enter new file name: ")
                path_handler.rename_path(title)
            while not path_handler.unique:
                title = input(
                    "⚠️   WARNING - File name already exist! Please enter another name: ")
                path_handler.rename_path(title)

        # Begin download images
        print("[ Downloading ]")
        if not os.path.exists(output_folder):
            os.mkdir(output_folder)
        if not os.path.exists(temp_folder):
            os.mkdir(temp_folder)
        if os.path.exists(path_handler.temp_path):
            shutil.rmtree(path_handler.temp_path)
        os.mkdir(path_handler.temp_path)
        images = []
        for p in range(dl_handler.pages):
            # Fetch each image link of the gallery
            sys.stdout.write(
                "\rDownloading page {}/{}...".format(p+1, dl_handler.pages))
            img_file = dl_handler.save_image(p+1, path_handler.temp_path)
            # Add to list of images for conversion later
            images.append(Image.open(img_file))
            sys.stdout.flush()
        print("Done!")

        # Convert to PDF
        print("[ Converting to PDF ]")
        pdf_handler = PDFHandler()
        pdf_handler.save_to_pdf(images, path_handler.final_path)
        print("Completed conversion!")

        # Remove temp images
        print("[ Removing Temp Data ]")
        shutil.rmtree(temp_folder)
        print(f'Saved at {path_handler.final_path}')
        if platform == "win32":
            print("Done!\n")
        else:
            print("Done ✅\n")

        try:
            log.write(f'{log_statement}[SUCCESS] {dl_handler.title}.\n')
        except:
            # In case a unicode character cannot be written to history log.
            log.write(f'{log_statement}[SUCCESS] [LOG ERROR] Title could not be recorded due to bad charaacter.\n')
    return


def get_command(output_folder, temp_folder, log):
    input_prompt = "Enter number(s): "
    num_input = input(input_prompt).split()
    while num_input[0] != "done":
        if (num_input[0] == "open"):
            open_folder(output_folder)
            print()
        elif (num_input[0] == "help"):
            show_help()
        else:
            process_queue(num_input, output_folder, temp_folder, log)
        # Ask for more input
        num_input = input(input_prompt).split()
    return


if __name__ == "__main__":
    # Start program
    print(f"[ nhentai downloader pdf ]\n")
    print('Enter \'help\' to see usage and commands\n')
    output_folder = os.path.join(os.getcwd(), 'hentai')
    all_temp = os.path.join(output_folder, 'temp')
    history_log = open('history.log', 'a+')
    try:
        get_command(output_folder, all_temp, history_log)
    except KeyboardInterrupt:
        print('\n\nStopping all queues (if any).')
    if os.path.exists(all_temp):
        shutil.rmtree(all_temp)

    history_log.close()
    print("\n---Program End---")


#Quelle: https://github.com/TacoAnime69/nh-pdf-downloader
