if [ ! "$(ls "$WORK_INPUTS_PATH")" ]
then 
        echo "Empty files"
        exit 0
fi

gunzip $WORK_INPUTS_PATH/*.gz
FIRST_FILE=true
for file in $(ls "$WORK_INPUTS_PATH"); do
        if [[ $file =~ (_META_) ]]
        then
                mv "${WORK_INPUTS_PATH}/${file}" "${WORK_OUTPUTS_PATH}/${file}"
        elif [[ "$FIRST_FILE" == "true" ]]
        then
                OUTFILE=$file
                cat "${WORK_INPUTS_PATH}/${file}" >> "${WORK_OUTPUTS_PATH}/${OUTFILE}"
                FIRST_FILE=false
        else
                tail -n+2 "${WORK_INPUTS_PATH}/${file}" >> "${WORK_OUTPUTS_PATH}/${OUTFILE}"
        fi
done

gzip "${WORK_OUTPUTS_PATH}/${OUTFILE}"
